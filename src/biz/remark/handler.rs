use actix_web::{HttpResponse, post, Error, web, HttpRequest, get};
use crate::AppState;
use crate::biz::courier::{Courier, HappyCourier, PaginateQuery, SadCourier};
use super::{courier, recorder};
use crate::biz::internal;
use crate::biz::internal::{get_pg, MAX_PAGE_SIZE, MIN_PAGE_SIZE};

#[post("")]
pub async fn create_remark(req: HttpRequest, app_state: web::Data<AppState>, req_body: web::Json<courier::RemarkCourier>) -> Result<HttpResponse, Error> {
    let user_id = internal::extract_user_id(req)?;

    let remark_courier = req_body.into_inner();

    let client = get_pg(&app_state).await?;

    let remark_record = recorder::insert(&client, remark_courier, user_id).await?;

    Ok(
        HttpResponse::Created().json(
            HappyCourier::build()
                .message("Success to create remark")
                .data(remark_record)
                .done()
        )
    )
}


#[get("/paginated/{parent_id}")]
pub async fn read_remark_paginated(app_state: web::Data<AppState>, path: web::Path<i64>, paginate_query: web::Query<PaginateQuery>) -> Result<HttpResponse, Error> {
    let client = get_pg(&app_state).await?;

    let paginate = paginate_query.into_inner();
    let parent_id = path.into_inner();

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

    let total_record = recorder::count(&client, parent_id).await?;

    if paginate.page_number > (total_record / paginate.page_size + 1) {
        return Ok(HttpResponse::BadRequest().json(
            SadCourier::brief("Page number is too big")
        ));
    }


    let remark_records = recorder::select_paginated(
        &client,
        parent_id,
        paginate.page_number,
        paginate.page_size,
    )
        .await?;

    Ok(
        HttpResponse::Ok().json(
            Courier::build()
                .message("Success to get remark data")
                .data(
                    remark_records
                )
                .extra(total_record)
                .done()
        )
    )
}
