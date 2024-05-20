use actix_web::{Error, get, HttpMessage, HttpRequest, HttpResponse, post, web};
use crate::AppState;
use crate::biz::account::recorder::{Account, query_account_by_id};
use crate::infra::error::biz_err::BizError;
use deadpool_postgres::{Client as PgClient};
use crate::biz::user::communicaotr::{ReqBodyForUpdatingUser, RespBodyForGettingUser, RespBodyForUpdatingUser};
use crate::biz::user::recorder::update_account;
use crate::infra::middleware::jwt::Claims;


#[get("")]
async fn get_user_info(req: HttpRequest, app_state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let pg_client: PgClient = app_state.pool.get().await.map_err(BizError::PoolError)?;

    let user_id = req.extensions()
        .get::<Claims>()
        .ok_or_else(|| BizError::JwtError)?
        .sub;

    let queried_account = query_account_by_id(&pg_client, user_id).await?;

    let resp: RespBodyForGettingUser = queried_account.into();

    Ok(
        HttpResponse::Ok()
            .json(
                resp
            )
    )
}

#[post("")]
async fn update_user_info(req: HttpRequest, app_state: web::Data<AppState>, req_body: web::Json<ReqBodyForUpdatingUser>) -> Result<HttpResponse, Error> {
    let pg_client: PgClient = app_state.pool.get().await.map_err(BizError::PoolError)?;

    let mut info_to_update = req_body.into_inner();

    let user_id = req.extensions()
        .get::<Claims>()
        .ok_or_else(|| BizError::JwtError)?
        .sub;

    info_to_update.user_id = user_id;

    let updated_user_info = update_account(&pg_client, &info_to_update.into()).await?;

    let resp: RespBodyForUpdatingUser = updated_user_info.into();

    Ok(
        HttpResponse::Ok()
            .json(
                resp
            )
    )
}
