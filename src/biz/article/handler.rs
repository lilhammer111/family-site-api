use actix_web::{HttpResponse, post, Error, web, HttpRequest, get};
use log::debug;
use crate::AppState;
use crate::biz::courier::HappyCourier;
use super::{courier, recorder};
use crate::biz::internal;
use crate::biz::internal::get_pg;

#[post("")]
pub async fn create_article(req: HttpRequest, app_state: web::Data<AppState>, req_body: web::Json<courier::ArticleCourier>) -> Result<HttpResponse, Error> {
    let user_id = internal::extract_user_id(req)?;

    let article_courier = req_body.into_inner();

    let client = get_pg(&app_state).await?;

    let article_record = recorder::insert(&client, article_courier, user_id).await?;

    Ok(
        HttpResponse::Created().json(
            HappyCourier::build()
                .message("Success to create article")
                .data(article_record)
                .done()
        )
    )
}

#[get("/owned")]
pub async fn read_owned_article(req: HttpRequest, app_state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let user_id = internal::extract_user_id(req)?;

    let client = get_pg(&app_state).await?;

    let article_record = recorder::select_by_id(&client, user_id).await?;

    Ok(
        HttpResponse::Ok().json(
            HappyCourier::build()
                .message("Success to get all article for the user")
                .data(article_record)
                .done()
        )
    )
}
