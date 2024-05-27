use std::ops::Add;
use actix_web::{Error, HttpRequest, HttpResponse, post, web};
use actix_web::cookie::{Cookie, SameSite};
use actix_web::cookie::time::OffsetDateTime;
use bcrypt::verify;
use chrono::{Local, TimeDelta};
use jsonwebtoken::{encode, EncodingKey, Header};
use log::{debug};
use crate::AppState;
use crate::biz::account::courier::ReqBodyForAuth;
use crate::biz::courier::{Courier, SadCourier};
use crate::biz::internal::get_pg;
use crate::infra::error::biz::BizKind::AuthorizationFailed;
use crate::infra::error::error::ServiceError;
use crate::infra::error::error::Kind::{BizError, InfraError};
use super::recorder::{select, add_account};
use crate::infra::middleware::jwt::{Claims, JWT_AUTH_KEY};

const TOKEN_SPAN: i64 = 30;

fn generate_token(id: i64, jwt_secret: &[u8], expired_at: i64) -> Result<String, ServiceError> {
    let claims = Claims {
        exp: expired_at,
        sub: id,
    };


    let jwt_token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret),
    )?;

    Ok(jwt_token)
}

fn generate_cookie(token: &str, expires: i64) -> Result<Cookie, ServiceError> {
    let expires = OffsetDateTime::from_unix_timestamp(expires)
        .map_err(|err| {
            ServiceError::build()
                .belong(InfraError)
                .because(Box::new(err))
                .done()
        })?;

    let cookie = Cookie::build(JWT_AUTH_KEY, token)
        .expires(expires)
        .domain("127.0.0.1")
        .path("/")
        .secure(true)
        .same_site(SameSite::None)
        .http_only(true)
        .finish();

    Ok(cookie)
}

#[post("/login")]
async fn login(app_state: web::Data<AppState>, body: web::Json<ReqBodyForAuth>, req: HttpRequest) -> Result<HttpResponse, Error> {
    debug!("login req: {:?}", req);
    let req = body.into_inner();

    let pg_client = get_pg(&app_state).await?;

    let queried_account = select(&pg_client, &req.username)
        .await?
        .pop()
        .ok_or(
            ServiceError::build()
                .belong(BizError(AuthorizationFailed))
                .message("User does not exist")
                .done()
        )?;


    match verify(req.password, &queried_account.password) {
        Ok(pwd_is_correct) => {
            if pwd_is_correct {
                let expires = Local::now().add(TimeDelta::hours(TOKEN_SPAN)).timestamp();

                let token = generate_token(queried_account.id, app_state.jwt_secret.as_bytes(), expires)?;

                let cookie = generate_cookie(&token, expires)?;

                Ok(
                    HttpResponse::Ok()
                        .cookie(cookie)
                        .json(
                            SadCourier::brief("login success")
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
async fn register(app_state: web::Data<AppState>, account_json: web::Json<ReqBodyForAuth>) -> Result<HttpResponse, Error> {
    let body = account_json.into_inner();
    let client = get_pg(&app_state).await?;

    let queried_account = select(&client, &body.username)
        .await?;

    if queried_account.is_empty() {
        let account_record = add_account(&client, &body.username, &body.password).await?;

        let expires = Local::now().add(TimeDelta::hours(TOKEN_SPAN)).timestamp();

        let token = generate_token(account_record.id, app_state.jwt_secret.as_bytes(), expires)?;

        let cookie = generate_cookie(&token, expires)?;

        Ok(
            HttpResponse::Created()
                .cookie(cookie)
                .json(
                    Courier::brief("Registration succeed")
                )
        )
    } else {
        Ok(
            HttpResponse::Conflict().json(
                Courier::brief("Username exists")
            )
        )
    }
}
