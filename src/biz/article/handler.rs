use actix_web::{HttpResponse, post, Error, web, HttpRequest, get};
use crate::AppState;
use crate::biz::courier::{Courier, HappyCourier, PaginateQuery, SadCourier};
use super::{courier, recorder};
use crate::biz::internal;
use crate::biz::internal::{get_pg, MAX_PAGE_SIZE, MIN_PAGE_SIZE};

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
pub async fn read_article_owned(req: HttpRequest, app_state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let user_id = internal::extract_user_id(req)?;

    let client = get_pg(&app_state).await?;

    let article_record = recorder::select_by_author_id(&client, user_id).await?;

    Ok(
        HttpResponse::Ok().json(
            HappyCourier::build()
                .message("Success to get all article for the user")
                .data(article_record)
                .done()
        )
    )
}

#[get("/paginated")]
pub async fn read_article_paginated(app_state: web::Data<AppState>, paginate_query: web::Query<PaginateQuery>) -> Result<HttpResponse, Error> {
    let client = get_pg(&app_state).await?;

    let paginate = paginate_query.into_inner();
    
    // params validation
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


    let article_records = recorder::select_paginated(
        &client,
        paginate.page_number,
        paginate.page_size,
    )
        .await?;

    Ok(
        HttpResponse::Ok().json(
            Courier::build()
                .message("Success to get article data")
                .data(
                    article_records
                )
                .extra(total_record)
                .done()
        )
    )
}
