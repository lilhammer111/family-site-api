use actix_web::{Error, get, HttpRequest, HttpResponse, post, web};
use crate::AppState;
use crate::biz::courier::{Courier, HappyCourier, PaginateQuery, SadCourier};
use crate::biz::internal::{extract_user_id, get_pg, MAX_PAGE_SIZE, MIN_PAGE_SIZE};
use crate::biz::wish::courier::{WishJson, WishResp};
use crate::biz::wish::recorder;

#[post("")]
pub async fn create_wish(req: HttpRequest, app_state: web::Data<AppState>, body: web::Json<WishJson>) -> Result<HttpResponse, Error> {
    let pg_client = get_pg(&app_state).await?;

    let user_id = extract_user_id(req)?;

    let wish_json = body.into_inner();

    if wish_json.content.is_empty() {
        return Ok(
            HttpResponse::BadRequest().json(
                HappyCourier::build()
                    .message("Wish content is empty")
                    .data("")
                    .done()
            )
        );
    }

    let wish_record = recorder::insert(
        &pg_client,
        user_id,
        &wish_json.content,
    ).await?;

    Ok(
        HttpResponse::Created()
            .json(
                HappyCourier::<WishResp>::build()
                    .message("Success to create the wish")
                    .data(
                        wish_record.into()
                    )
            )
    )
}

#[get("/paginated")]
pub async fn get_paginated_wish(app_state: web::Data<AppState>, wish_params: web::Query<PaginateQuery>) -> Result<HttpResponse, Error> {
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