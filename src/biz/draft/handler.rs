use actix_web::{Error, get, HttpRequest, HttpResponse, post, web};
use crate::AppState;
use super::{courier, recorder};
use crate::biz::courier::{HappyCourier, SadCourier};
use crate::biz::internal;
use crate::biz::internal::get_pg;

#[post("")]
pub async fn create_draft(req: HttpRequest, app_state: web::Data<AppState>, req_body: web::Json<courier::DraftCourier>) -> Result<HttpResponse, Error> {
    let user_id = internal::extract_user_id(req)?;

    let draft_courier = req_body.into_inner();

    let client = get_pg(&app_state).await?;

    recorder::insert(&client, user_id, draft_courier.text).await?;

    Ok(
        HttpResponse::Ok().json(
            SadCourier::brief("Success to save draft")
        )
    )
}

#[get("")]
pub async fn read_draft_owned(req: HttpRequest, app_state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let user_id = internal::extract_user_id(req)?;

    let client = get_pg(&app_state).await?;

    let res = recorder::select(&client, user_id).await?;

    if res.is_empty() {
        Ok(
            HttpResponse::NotFound().json(
                SadCourier::brief("The user has no draft")
            )
        )
    } else {
        Ok(
            HttpResponse::Ok().json(
                HappyCourier::build()
                    .message("Success to find draft")
                    .data(res.get(0))
                    .done()
            )
        )
    }
}