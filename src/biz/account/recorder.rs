use bcrypt::{DEFAULT_COST, hash};
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use deadpool_postgres::{Client as PgClient, GenericClient};
use tokio_pg_mapper::FromTokioPostgresRow;
use chrono::{NaiveDate};
// use chrono::NaiveDateTime;
use crate::infra::error::BizError;


#[derive(Deserialize, PostgresMapper, Serialize, Debug)]
#[pg_mapper(table = "account")]
pub struct Account {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub mobile: Option<String>,
    pub email: Option<String>,
    pub avatar_url: Option<String>, // Optional since the avatar_url can be NULL
    pub pronouns: Option<String>, // Optional since the pronouns can be NULL
    pub birthday: Option<NaiveDate>, // Optional and using NaiveDate for DATE type
    pub industry: Option<String>, // Optional since the industry can be NULL
    pub location: Option<String>, // Optional since the location can be NULL
    pub social_account: Vec<String>, // Using Vec<String> for TEXT[] type
    // todo: 些许复杂明天再写
    pub create_at: NaiveDate, // Using NaiveDateTime for TIMESTAMP
    pub update_at: NaiveDate, // Using NaiveDateTime for TIMESTAMP
}

pub async fn find_account(pc: &PgClient, username: &str) -> Result<Account, BizError> {
    let stmt = "SELECT * FROM account WHERE username = $1";

    let row = pc
        .query_one(stmt, &[&username])
        .await?;

    Account::from_row_ref(&row).map_err(Into::into)
}

pub async fn add_account(pc: &PgClient, account: &Account) -> Result<Account, BizError> {
    let hashed_pwd = hash(&account.password, DEFAULT_COST)?;

    let stmt = "INSERT INTO account(username, password) VALUES ($1, $2) RETURNING *";

    let row = pc
        .query_one(stmt, &[&account.username, &hashed_pwd])
        .await?;

    Account::from_row_ref(&row).map_err(Into::into)
}

pub async fn update_account(pc: &PgClient, account: &Account) -> Result<Account, BizError> {
    let hashed_pwd = hash(&account.password, DEFAULT_COST)?;

    let stmt = r#"
        UPDATE account
        SET
            username = $2,
            password = $3,
            mobile = $4,
            email = $5,
            avatar_url = $6,
            pronouns = $7,
            birthday = $8,
            industry = $9,
            location = $10,
            social_account = $11,
            update_at = NOW()
        WHERE id = $1
        RETURNING *;
    "#;

    let row = pc
        .query_one(stmt, &[
            &account.id, // id
            &account.username, // username
            &hashed_pwd, // password
            &account.mobile, // mobile
            &account.email, // email
            &account.avatar_url, // avatar_url
            &account.pronouns, // pronouns
            &account.birthday, // birthday
            &account.industry, // industry
            &account.location, // location
            &account.social_account, // social_account (ensure this is serialized appropriately)
        ])
        .await?;

    Account::from_row_ref(&row).map_err(Into::into)
}