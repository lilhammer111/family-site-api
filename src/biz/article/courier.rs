use serde::{Deserialize, Serialize};
use crate::biz::courier::PaginateQuery;

#[derive(Serialize, Debug, Deserialize)]
pub struct Article {
    pub cover_url: String,
    pub title: String,
    pub author_id: i64,
    pub summary: Option<String>,
    pub text: String,
}

pub struct ArticleFilter {
    pub paginate: PaginateQuery,
}