use std::collections::HashMap;
use actix_web::{HttpResponse, Error, web, get};
use crate::AppState;
use crate::biz::article_category::recorder::{select_all_category, select_distinct_level};
use crate::biz::courier::{Courier};
use crate::biz::internal::get_pg;

#[get("")]
pub async fn read_all_category(app_state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let client = get_pg(&app_state).await?;

    let category_records = select_all_category(&client).await?;

    let level_counts = select_distinct_level(&client).await?;

    Ok(
        HttpResponse::Ok().json(
            Courier::build()
                .message("Success to query category data")
                .data(category_records)
                .extra(level_counts)
                .done()
        )
    )
}
