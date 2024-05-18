use std::ops::Add;
use actix_web::{Error, HttpRequest, HttpResponse, post, web};
use bcrypt::verify;
use chrono::{Local, TimeDelta};
use deadpool_postgres::{Client as PgClient, Pool};
use jsonwebtoken::{encode, EncodingKey, Header};
use log::debug;
use crate::biz::account::communicator::{CommMessage, EmptyData, AccountCommunicator, AccountRespData};
use super::recorder::{Account, add_account, find_account};
use crate::infra::error::BizError;
use crate::infra::config::Settings;
use crate::infra::middleware::jwt::Claims;


const TOKEN_SPAN: i64 = 5;

fn generate_token(id: i64, jwt_secret: &[u8]) -> Result<String, BizError> {
    let expires = Local::now().add(TimeDelta::minutes(TOKEN_SPAN)).timestamp();
    let claims = Claims {
        exp: expires,
        sub: id,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret),
    ).map_err(|e| BizError::JwtError)
}

#[post("/login")]
async fn login(settings: web::Data<Settings>, pg_pool: web::Data<Pool>, account_json: web::Json<Account>) -> Result<HttpResponse, Error> {
    let req = account_json.into_inner();
    let pg_client: PgClient = pg_pool.get().await.map_err(BizError::PoolError)?;

    debug!("pg_client is {:#?}", pg_client);

    let queried_account = find_account(&pg_client, &req.username).await?;


    match verify(req.password, &queried_account.password) {
        Ok(pwd_is_correct) => {
            if pwd_is_correct {
                let token = generate_token(queried_account.id, settings.jwt_secret.as_bytes())?;

                let resp_data = AccountRespData {
                    user_id: queried_account.id,
                    username: queried_account.username,
                };

                let login_comm = AccountCommunicator::new(
                    CommMessage::Success,
                    resp_data,
                    token.as_str(),
                );

                Ok(HttpResponse::Ok().json(login_comm))
            } else {
                let login_comm = AccountCommunicator::new(
                    CommMessage::Fail,
                    EmptyData,
                    "",
                );

                Ok(HttpResponse::Unauthorized().json(login_comm))
            }
        }
        Err(_) => Ok(HttpResponse::InternalServerError().finish())
    }
}


#[post("/register")]
async fn register(settings: web::Data<Settings>, pg_pool: web::Data<Pool>, account_json: web::Json<Account>) -> Result<HttpResponse, Error> {
    let req = account_json.into_inner();
    let pg_client: PgClient = pg_pool.get().await.map_err(BizError::PoolError)?;

    match find_account(&pg_client, &req.username).await {
        Err(e) => {
            if matches!(e, BizError::NotFound) {
                let created_account = add_account(&pg_client, &req).await?;

                let token = generate_token(created_account.id, settings.jwt_secret.as_bytes())?;

                let resp_data = AccountRespData {
                    user_id: created_account.id,
                    username: created_account.username,
                };

                let register_comm = AccountCommunicator::new(
                    CommMessage::Success,
                    resp_data,
                    token.as_str(),
                );

                Ok(HttpResponse::Ok().json(register_comm))
            } else {
                e.into()
            }
        }
        Ok(_) => {
            Ok(HttpResponse::Conflict().body("{}"))
        }
    }
}

