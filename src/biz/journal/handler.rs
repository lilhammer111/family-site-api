use actix_web::{Error, get, HttpRequest, HttpResponse, post, web};
use crate::AppState;
use crate::biz::courier::{Courier, HappyCourier, SadCourier};
use crate::biz::internal::{extract_user_id, get_pg, MAX_PAGE_SIZE, MIN_PAGE_SIZE};
use crate::biz::journal::courier::{JournalJson, WishQuery, WishResp};
use crate::biz::journal::recorder::JournalRecord;
use super::recorder;

#[post("")]
pub async fn create_journal(req: HttpRequest, app_state: web::Data<AppState>, body: web::Json<JournalJson>) -> Result<HttpResponse, Error> {
    let pg_client = get_pg(&app_state).await?;

    let user_id = extract_user_id(req)?;

    let journal_body = body.into_inner();

    if journal_body.content.is_empty() || journal_body.title.is_empty() {
        return Ok(
            HttpResponse::BadRequest().json(
                SadCourier::brief("Journal content or title is empty")
            )
        );
    }

    let journal_record = recorder::insert(
        &pg_client,
        &journal_body.title,
        &journal_body.content,
        &journal_body.images.iter().map(|image_url| image_url.as_str()).collect(),
    ).await?;

    Ok(
        HttpResponse::Created()
            .json(
                HappyCourier::<JournalRecord>::build()
                    .message("Success to create journal")
                    .data(journal_record)
                    .done()
            )
    )
}

#[get("/paginated")]
pub async fn read_paginated_journal(app_state: web::Data<AppState>, wish_params: web::Query<WishQuery>) -> Result<HttpResponse, Error> {
    let pg_client = get_pg(&app_state).await?;

    // params validation
    let wish_params = wish_params.into_inner();

    if wish_params.page_size < MIN_PAGE_SIZE {
        return Ok(HttpResponse::BadRequest().json(
            SadCourier::brief("Page size is too small")
        ));
    }

    if wish_params.page_size > MAX_PAGE_SIZE {
        return Ok(HttpResponse::BadRequest().json(
            SadCourier::brief("Page size is too big")
        ));
    }

    let total_record = recorder::count(&pg_client).await?;

    if wish_params.page_number > (total_record / wish_params.page_size + 1) {
        return Ok(HttpResponse::BadRequest().json(
            SadCourier::brief("Page number is too big")
        ));
    }


    let wish_records = recorder::select_many(&pg_client, wish_params.page_number, wish_params.page_size)
        .await?;

    let wish_resp = wish_records.into_iter()
        .map(WishResp::from)
        .collect::<Vec<WishResp>>();

    Ok(
        HttpResponse::Ok().json(
            Courier::<Vec<WishResp>, i64>::build()
                .message("Success to get wish data")
                .data(
                    wish_resp
                )
                .extra(total_record)
                .done()
        )
    )
}