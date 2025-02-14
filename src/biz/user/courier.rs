use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use crate::biz::user::recorder::UserRecorder;

#[derive(Serialize, Debug, Deserialize)]
pub struct UserQuery {
    pub user_ids: Vec<i64>,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct UserJson {
    pub username: String,
    pub mobile: Option<String>,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
    pub pronouns: Option<String>,
    pub birthday: Option<NaiveDate>,
    pub industry: Option<String>,
    pub location: Option<String>,
    pub social_account: Option<Vec<String>>,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct UserResp {
    pub id: i64,
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

impl From<UserRecorder> for UserResp {
    fn from(value: UserRecorder) -> Self {
        UserResp {
            id: value.id,
            username: value.username,
            mobile: value.mobile,
            email: value.email,
            avatar_url: value.avatar_url,
            pronouns: value.pronouns,
            birthday: value.birthday,
            industry: value.industry,
            location: value.location,
            social_account: value.social_account,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl Into<UserRecorder> for UserJson {
    fn into(self) -> UserRecorder {
        UserRecorder {
            id: 0,
            username: self.username,
            password: "".to_string(),
            mobile: self.mobile,
            email: self.email,
            avatar_url: self.avatar_url,
            pronouns: self.pronouns,
            birthday: self.birthday,
            industry: self.industry,
            location: self.location,
            social_account: self.social_account,
            created_at: NaiveDateTime::default(),
            updated_at: NaiveDateTime::default(),
        }
    }
}

#[derive(Serialize, Debug, Deserialize, Default)]
pub struct UserPublicCourier {
    pub username: String,
    pub avatar_url: Option<String>,
}

impl From<UserRecorder> for UserPublicCourier {
    fn from(ur: UserRecorder) -> Self {
        UserPublicCourier {
            username: ur.username,
            avatar_url: ur.avatar_url,
        }
    }
}