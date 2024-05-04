use actix_web::{Error, HttpResponse, post, web};
use deadpool_postgres::{Client as PgClient, Pool};
use serde::Serialize;
use crate::api::account::model::{Account, add_account, get_account_pwd};
use crate::error::DbError;

#[derive(Serialize)]
struct Empty {}

#[post("/login")]
async fn login(pg_pool: web::Data<Pool>, account_json: web::Json<Account>) -> Result<HttpResponse, Error> {
    let account = account_json.into_inner();
    let pc: PgClient = pg_pool.get().await.map_err(DbError::PoolError)?;

    let pwd = get_account_pwd(&pc, &account.account_name).await?;

    if pwd == account.password {
        Ok(HttpResponse::Ok().json(Empty {}))
    } else {
        Ok(HttpResponse::Unauthorized().json(Empty {}))
    }
}


#[post("/register")]
async fn register(pg_pool: web::Data<Pool>, account_json: web::Json<Account>) -> Result<HttpResponse, Error> {
    let account = account_json.into_inner();
    let pc: PgClient = pg_pool.get().await.map_err(DbError::PoolError)?;

    match get_account_pwd(&pc, &account.account_name).await {
        Err(_) => {
            let account = add_account(&pc, account).await?;
            Ok(HttpResponse::Ok().json(account))
        },
        Ok(_) => {
            Ok(HttpResponse::Conflict().json(Empty {}))
        }
    }
}