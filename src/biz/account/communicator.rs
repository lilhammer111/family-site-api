use std::fmt::Debug;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct RespBodyForAuth {
    pub user_id: i64,
    pub username: String,
}


#[derive(Deserialize)]
pub struct ReqBodyForAuth {
    pub username: String,
    pub password: String,
}