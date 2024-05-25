use actix_web::{Error, get, HttpRequest, HttpResponse, post, web};
use crate::AppState;
use log::debug;
use crate::biz::courier::{HappyCourier};
use crate::biz::user::communicator::{UserQuery, UserJson, UserResp};
use crate::biz::user::recorder::{query_account_by_id, select_many, update_account};
use crate::biz::internal::{extract_user_id, get_pg};
use serde_querystring_actix;
use serde_querystring_actix::QueryString;

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

#[get("/batch")]
async fn get_user_info_in_batches(app_state: web::Data<AppState>, QueryString(user_query): QueryString<UserQuery>) -> Result<HttpResponse, Error> {
    let client = get_pg(&app_state).await?;

    let user_records = select_many(&client, user_query.user_ids.as_slice()).await?;

    let resp = user_records.into_iter().map(Into::into).collect::<Vec<UserResp>>();

    Ok(HttpResponse::Ok().json(
        HappyCourier::<Vec<UserResp>>::build()
            .message("Success to get user information in batches")
            .data(resp)
            .done()
    ))
}

#[post("")]
async fn update_user_info(req: HttpRequest, app_state: web::Data<AppState>, req_body: web::Json<UserJson>) -> Result<HttpResponse, Error> {
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
