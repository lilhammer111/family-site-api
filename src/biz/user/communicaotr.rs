use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use crate::biz::account::recorder::Account;
use crate::infra::init::Initializer;

#[derive(Deserialize, Debug)]
pub struct ReqBodyForUpdatingUser {
    pub user_id: i64,
    pub username: String,
    pub mobile: Option<String>,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
    pub pronouns: Option<String>,
    pub birthday: Option<NaiveDate>,
    pub industry: Option<String>,
    pub location: Option<String>,
    pub social_account: Option<Vec<String>>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Debug)]
pub struct RespBodyForGettingUser {
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

pub(crate) type RespBodyForUpdatingUser = RespBodyForGettingUser;

impl From<Account> for RespBodyForGettingUser {
    fn from(account: Account) -> Self {
        RespBodyForGettingUser {
            user_id: account.id,
            username: account.username,
            mobile: account.mobile,
            email: account.email,
            avatar_url: account.avatar_url,
            pronouns: account.pronouns,
            birthday: account.birthday,
            industry: account.industry,
            location: account.location,
            social_account: account.social_account,
            created_at: account.created_at,
            updated_at: account.updated_at,
        }
    }
}

impl Into<Account> for

