use actix_web::{Error, get, HttpRequest, HttpResponse, post, web};
use crate::AppState;
use log::debug;
use crate::biz::user::communicator::{UserReq, UserResp};
use crate::biz::user::recorder::{query_account_by_id, update_account};
use crate::biz::internal::{extract_user_id, get_pg};

#[get("")]
async fn get_user_info(req: HttpRequest, app_state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let pg_client = get_pg(&app_state).await?;

    let user_id = extract_user_id(req)?;

    let queried_user = query_account_by_id(&pg_client, user_id).await?;

    let resp: UserResp = queried_user.into();

    Ok(
        HttpResponse::Ok()
            .json(
                resp
            )
    )
}

#[post("")]
async fn update_user_info(req: HttpRequest, app_state: web::Data<AppState>, req_body: web::Json<UserReq>) -> Result<HttpResponse, Error> {
    debug!("update user req body: {:?}", req_body);

    let pg_client = get_pg(&app_state).await?;

    let info_to_update = req_body.into_inner();

    let user_id = extract_user_id(req)?;

    let updated_user_info = update_account(&pg_client, user_id, info_to_update).await?;

    let resp: UserResp = updated_user_info.into();

    Ok(
        HttpResponse::Ok()
            .json(
                resp
            )
    )
}
