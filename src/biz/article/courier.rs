use serde::{Deserialize, Serialize};
use crate::biz::courier::PaginateQuery;

#[derive(Serialize, Debug, Deserialize)]
pub struct ArticleCourier {
    pub kind: String,
    pub cover_url: Option<String>,
    pub title: String,
    pub category_id: Option<i32>,
    pub summary: Option<String>,
    pub text: Option<String>,
    pub text_url: Option<String>,
}

pub struct ArticleFilter {
    pub paginate: PaginateQuery,
}