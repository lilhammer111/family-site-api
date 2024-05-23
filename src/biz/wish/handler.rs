use actix_web::{Error, get, HttpMessage, HttpRequest, HttpResponse, post, web};
use crate::AppState;
use crate::infra::error::biz_err::BizError;
use deadpool_postgres::{Client as PgClient};
use crate::biz::base_comm::{Communicator, Empty};
use crate::biz::wish::communicator::{WishJson, WishQuery, WishResp};
use crate::biz::wish::recorder;
use crate::infra::middleware::jwt::Claims;


#[post("")]
pub async fn create_wish(req: HttpRequest, app_state: web::Data<AppState>, body: web::Json<WishJson>) -> Result<HttpResponse, Error> {
    let pg_client: PgClient = app_state.pool.get().await?;

    let user_id = req.extensions()
        .get::<Claims>()
        .ok_or_else(|| BizError::JwtError)?
        .sub;

    let wish_json = body.into_inner();

    if wish_json.content.is_empty() {
        return Ok(
            HttpResponse::BadRequest().json(
                Communicator::builder()
                    .message("Wish content is empty")
                    .data("")
                    .build()
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
                Communicator::<WishResp>::builder()
                    .message("Success to create the wish")
                    .data(
                        wish_record.into()
                    )
            )
    )
}

const MAX_PAGE_SIZE: i64 = 20;
const MIN_PAGE_SIZE: i64 = 10;

#[get("/paginated")]
pub async fn get_paginated_wish(app_state: web::Data<AppState>, wish_params: web::Query<WishQuery>) -> Result<HttpResponse, Error> {
    let client: PgClient = app_state.pool.get().await?;

    // params validation
    let wish_params = wish_params.into_inner();

    if wish_params.page_size <= MIN_PAGE_SIZE {
        return Ok(HttpResponse::BadRequest().json(
            Communicator::<Empty>::builder()
                .message("Page size is too small")
                .build()
        ));
    }

    if wish_params.page_size >= MAX_PAGE_SIZE {
        return Ok(HttpResponse::BadRequest().json(
            Communicator::<Empty>::builder()
                .message("Page size is too big")
                .build()
        ));
    }

    let total_record = recorder::count(&client).await?;

    if wish_params.page_number > (total_record / wish_params.page_size + 1) {
        return Ok(HttpResponse::BadRequest().json(
            Communicator::<Empty>::builder()
                .message("Page number is too big")
                .build()
        ));
    }

    let wish_records =  recorder::select_many(&client,wish_params.page_number, wish_params.page_size)
        .await?;

    let wish_resp = wish_records.into_iter()
        .map(WishResp::from)
        .collect::<Vec<WishResp>>();

    Ok(
        HttpResponse::Ok().json(
            Communicator::<Vec<WishResp>>::builder()
                .message("Success to get wish data")
                .data(
                    wish_resp
                )
        )
    )
}