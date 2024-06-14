use serde::{Deserialize, Serialize};
// use crate::biz::courier::PaginateQuery;

#[derive(Serialize, Debug, Deserialize)]
pub struct ArticleCourier {
    pub kind: String,
    pub tags: Vec<Option<String>>,
    pub is_trending: Option<bool>,
    pub is_insight: Option<bool>,
    pub is_recommend: Option<bool>,
    pub cover_url: Option<String>,
    pub title: String,
    pub summary: Option<String>,
    pub text: Option<String>,
    pub text_url: Option<String>,
}

// pub struct ArticleFilter {
//     pub paginate: PaginateQuery,
// }