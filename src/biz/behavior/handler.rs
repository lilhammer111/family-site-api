use actix_web::{Error, get, HttpResponse, post, web};
use crate::AppState;
use crate::biz::courier::{Courier, HappyCourier, PaginateQuery, SadCourier};
use crate::biz::internal::{get_pg, MAX_PAGE_SIZE, MIN_PAGE_SIZE};
use super::{courier, recorder};

#[post("")]
pub async fn create_behavior(
    app_state: web::Data<AppState>,
    req_body: web::Json<courier::Behavior>,
) -> Result<HttpResponse, Error> {
    let pg_client = get_pg(&app_state).await?;

    let behavior_parcel = req_body.into_inner();

    //validate
    behavior_parcel.validate()?;

    let behavior_record = recorder::insert(
        &pg_client,
        &behavior_parcel,
    ).await?;

    Ok(
        HttpResponse::Created()
            .json(
                HappyCourier::build()
                    .message("Success to create behavior")
                    .data(behavior_record)
                    .done()
            )
    )
}


#[get("/all")]
pub async fn read_all_behavior_record(app_state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let client = get_pg(&app_state).await?;
    let total_record = recorder::count(&client).await?;
    let diet_records = recorder::select_all(&client).await?;

    Ok(
        HttpResponse::Ok().json(
            Courier::build()
                .message("Success to get all behavior data")
                .data(
                    diet_records
                )
                .extra(total_record)
                .done()
        )
    )
}

#[get("/paginated")]
pub async fn read_paginated_behavior(app_state: web::Data<AppState>, paginate_query: web::Query<PaginateQuery>) -> Result<HttpResponse, Error> {
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


    let behavior_records = recorder::select_many(
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
                    behavior_records
                )
                .extra(total_record)
                .done()
        )
    )
}