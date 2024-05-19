use std::ops::Add;
use actix_web::{Error, get, HttpMessage, HttpRequest, HttpResponse, post, web};
use actix_web::cookie::{Cookie, SameSite};
use actix_web::cookie::time::OffsetDateTime;
use bcrypt::verify;
use chrono::{Local, TimeDelta};
use deadpool_postgres::{Client as PgClient};
use jsonwebtoken::{encode, EncodingKey, Header};
use log::{debug, error};
use crate::AppState;
use crate::biz::account::communicator::{Communicator, ReqDataForAuth, RespDataForGettingUser};
use super::recorder::{add_account, query_account, query_account_by_id};
use crate::infra::error::{biz_err::BizError, infra_err::InfraError};
use crate::infra::middleware::jwt::{Claims, JWT_AUTH_KEY};


const TOKEN_SPAN: i64 = 30;

fn generate_token(id: i64, jwt_secret: &[u8], expired_at: i64) -> Result<String, BizError> {
    let claims = Claims {
        exp: expired_at,
        sub: id,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret),
    ).map_err(|_| BizError::JwtError)
}

#[post("/login")]
async fn login(app_state: web::Data<AppState>, req: web::Json<ReqDataForAuth>) -> Result<HttpResponse, Error> {
    let req = req.into_inner();
    let pg_client: PgClient = app_state.pool.get().await.map_err(BizError::PoolError)?;

    debug!("pg_client is {:#?}", pg_client);

    let queried_account = query_account(&pg_client, &req.username).await?;


    match verify(req.password, &queried_account.password) {
        Ok(pwd_is_correct) => {
            if pwd_is_correct {
                let expires = Local::now().add(TimeDelta::hours(TOKEN_SPAN)).timestamp();

                let token = generate_token(queried_account.id, app_state.jwt_secret.as_bytes(), expires)?;

                let cookie = Cookie::build(JWT_AUTH_KEY, &token)
                    .expires(
                        OffsetDateTime::from_unix_timestamp(expires)
                            .map_err(|err| InfraError::DepError(err))?
                    )
                    .path("/")
                    .http_only(true)
                    .same_site(SameSite::Strict)
                    .finish();

                Ok(
                    HttpResponse::Ok()
                        .cookie(cookie)
                        .json(
                            Communicator {
                                message: "login success".to_string(),
                                data: "",
                            }
                        )
                )
            } else {
                Ok(HttpResponse::Unauthorized().finish())
            }
        }
        Err(_) => Ok(HttpResponse::InternalServerError().finish())
    }
}


#[post("/register")]
async fn register(app_state: web::Data<AppState>, account_json: web::Json<ReqDataForAuth>) -> Result<HttpResponse, Error> {
    let req = account_json.into_inner();
    let pg_client: PgClient = app_state.pool.get().await.map_err(BizError::PoolError)?;

    match query_account(&pg_client, &req.username).await {
        Err(e) => {
            debug!("e: {:#?}", e);

            if matches!(e, BizError::NotFound) {
                let created_account = add_account(&pg_client, &req.username, &req.password).await?;

                let expires = Local::now().add(TimeDelta::minutes(TOKEN_SPAN)).timestamp();

                let token = generate_token(created_account.id, app_state.jwt_secret.as_bytes(), expires)?;

                let cookie = Cookie::build(JWT_AUTH_KEY, &token)
                    .expires(
                        OffsetDateTime::from_unix_timestamp(expires)
                            .map_err(|err| InfraError::DepError(err))?
                    )
                    .path("/")
                    .http_only(true)
                    .same_site(SameSite::Strict)
                    .finish();

                Ok(
                    HttpResponse::Created()
                        .cookie(cookie)
                        .json(
                            Communicator {
                                message: "register success".to_string(),
                                data: "",
                            }
                        )
                )
            } else {
                error!("error 500: {}", e.to_string());

                Ok(HttpResponse::InternalServerError().json(""))
            }
        }
        Ok(_) => {
            Ok(HttpResponse::Conflict().body("{}"))
        }
    }
}

#[get("")]
async fn get_user_info(req: HttpRequest, app_state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let pg_client: PgClient = app_state.pool.get().await.map_err(BizError::PoolError)?;

    let user_id= req.extensions()
        .get::<Claims>()
        .ok_or_else(|| BizError::JwtError)?
        .sub;

    let queried_account = query_account_by_id(&pg_client, user_id).await?;


    Ok(HttpResponse::Ok().json(RespDataForGettingUser {
        user_id,
        username: queried_account.username,
        mobile: queried_account.mobile,
        email: queried_account.email,
        avatar_url: queried_account.avatar_url,
        pronouns: queried_account.pronouns,
        birthday: queried_account.birthday,
        industry: queried_account.industry,
        location: queried_account.location,
        social_account: queried_account.social_account,
        created_at: queried_account.created_at,
        updated_at: queried_account.updated_at,
    }, ))
}