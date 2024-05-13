use actix_web::{Error, HttpResponse, post, web};
use deadpool_postgres::{Client as PgClient, Pool};
use log::{debug, info};
use serde::Serialize;
use super::model::{Account, add_account, get_account_pwd};
use crate::error::DbError;

#[derive(Serialize)]
struct Empty {}

#[post("/login")]
async fn login(pg_pool: web::Data<Pool>, account_json: web::Json<Account>) -> Result<HttpResponse, Error> {
    let account = account_json.into_inner();
    let pc: PgClient = pg_pool.get().await.map_err(DbError::PoolError)?;

    debug!("pc is {:#?}", pc);

    let res = get_account_pwd(&pc, &account.username).await;

    match res {
        Ok(pwd) => {
            if pwd == account.password {
                Ok(HttpResponse::Ok().body("{}"))
            } else {
                Ok(HttpResponse::Unauthorized().body("{}"))
            }
        }
        Err(e) => {
            info!("Failed to verify authentication: {}",e);
            match e {
                DbError::NotFound => Ok(HttpResponse::Unauthorized().body("{}")),
                _ => { Ok(HttpResponse::InternalServerError().body("{}")) }
            }
        }
    }
}


#[post("/register")]
async fn register(pg_pool: web::Data<Pool>, account_json: web::Json<Account>) -> Result<HttpResponse, Error> {
    let account = account_json.into_inner();
    let pc: PgClient = pg_pool.get().await.map_err(DbError::PoolError)?;

    match get_account_pwd(&pc, &account.username).await {
        Err(_) => {
            let res = add_account(&pc, &account).await;
            match res {
                Ok(_) => Ok(HttpResponse::Created().body("{}")),
                Err(_) => Ok(HttpResponse::InternalServerError().body("{}")),
            }
        }
        Ok(_) => {
            Ok(HttpResponse::Conflict().json(Empty {}))
        }
    }
}