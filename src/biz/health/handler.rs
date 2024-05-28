use actix_web::{Error, get, HttpResponse, post, web};
use crate::AppState;
use crate::biz::courier::{Courier, HappyCourier, PaginateQuery, SadCourier};
use crate::biz::health::courier::HealthJson;
use crate::biz::internal::{get_pg, MAX_PAGE_SIZE, MIN_PAGE_SIZE};
use super::recorder;

#[post("")]
pub async fn create_health(app_state: web::Data<AppState>, body: web::Json<HealthJson>) -> Result<HttpResponse, Error> {
    let pg_client = get_pg(&app_state).await?;

    let health_body = body.into_inner();

    // validate
    if health_body.height < 0.0 || health_body.weight < 0.0 || health_body.teeth < 0|| health_body.head_circumference < 0.0 {
        return Ok(
            HttpResponse::BadRequest().json(
                SadCourier::brief("Journal content or title is empty")
            )
        );
    }

    // validate date todo

    let health_record = recorder::insert(
        &pg_client,
        health_body.height,
        health_body.weight,
        health_body.teeth,
        health_body.head_circumference,
        health_body.measurement_date
    ).await?;

    Ok(
        HttpResponse::Created()
            .json(
                HappyCourier::build()
                    .message("Success to create health")
                    .data(health_record)
                    .done()
            )
    )
}

#[get("/paginated")]
pub async fn read_paginated_health(app_state: web::Data<AppState>, paginate_query: web::Query<PaginateQuery>) -> Result<HttpResponse, Error> {
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


    let health_records = recorder::select_many(
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
                    health_records
                )
                .extra(total_record)
                .done()
        )
    )
}