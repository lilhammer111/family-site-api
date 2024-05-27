use actix_web::{Error, get, HttpResponse, post, web};
use crate::AppState;
use crate::biz::courier::{Courier, HappyCourier, PaginateQuery, SadCourier};
use crate::biz::internal::{get_pg, MAX_PAGE_SIZE, MIN_PAGE_SIZE};
use crate::biz::journal::courier::{JournalJson};
use super::recorder;

#[post("")]
pub async fn create_journal(app_state: web::Data<AppState>, body: web::Json<JournalJson>) -> Result<HttpResponse, Error> {
    let pg_client = get_pg(&app_state).await?;

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
        &journal_body.images.iter().map(|image_url| image_url.as_str()).collect::<Vec<&str>>(),
    ).await?;

    Ok(
        HttpResponse::Created()
            .json(
                HappyCourier::build()
                    .message("Success to create journal")
                    .data(journal_record)
                    .done()
            )
    )
}

#[get("/paginated")]
pub async fn read_paginated_journal(app_state: web::Data<AppState>, paginate_query: web::Query<PaginateQuery>) -> Result<HttpResponse, Error> {
    let client = get_pg(&app_state).await?;

    // params validation
    let paginate = paginate_query.into_inner();

    if paginate.page_size < MIN_PAGE_SIZE {
        return Ok(HttpResponse::BadRequest().json(
            SadCourier::brief("Page size is too small")
        ));
    }

    if paginate.page_size > MAX_PAGE_SIZE {
        return Ok(HttpResponse::BadRequest().json(
            SadCourier::brief("Page size is too big")
        ));
    }

    let total_record = recorder::count(&client).await?;

    if paginate.page_number > (total_record / paginate.page_size + 1) {
        return Ok(HttpResponse::BadRequest().json(
            SadCourier::brief("Page number is too big")
        ));
    }


    let journal_records = recorder::select_many(
        &client,
        paginate.page_number,
        paginate.page_size,
    )
        .await?;

    Ok(
        HttpResponse::Ok().json(
            Courier::build()
                .message("Success to get wish data")
                .data(
                    journal_records
                )
                .extra(total_record)
                .done()
        )
    )
}