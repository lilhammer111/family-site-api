use std::fmt::Debug;
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct RespDataForGettingUser {
    pub user_id: i64,
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pronouns: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthday: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub industry: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub social_account: Option<Vec<String>>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Debug)]
pub struct RespDataForAuth {
    pub user_id: i64,
    pub username: String,
}


#[derive(Deserialize)]
pub struct ReqDataForAuth {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Debug)]
pub struct Communicator<D> {
    pub(crate) message: String,
    pub(crate) data: D
}

