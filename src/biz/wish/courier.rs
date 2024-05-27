use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use crate::biz::wish::recorder::WishRecord;

#[derive(Serialize, Debug, Deserialize)]
pub struct WishJson {
    pub content: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct WishQuery {
    pub page_number: i64,
    pub page_size:i64,
}

#[derive(Serialize, Debug, Deserialize, Default)]
pub struct WishResp {
    pub id: i64,
    pub user_id: i64,
    pub content: String,
    pub created_at: NaiveDateTime,
}

impl From<WishRecord> for WishResp {
    fn from(wish_record: WishRecord) -> Self {
        WishResp {
            id: wish_record.id,
            user_id: wish_record.user_id,
            content: wish_record.content,
            created_at: wish_record.created_at,
        }
    }
}
