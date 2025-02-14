use bcrypt::{DEFAULT_COST, hash};
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use deadpool_postgres::{Client as PgClient, GenericClient};
use tokio_pg_mapper::FromTokioPostgresRow;
use chrono::{NaiveDate};
use chrono::NaiveDateTime;
use log::debug;
use crate::infra::error::error::ServiceError;

#[derive(Deserialize, PostgresMapper, Debug, Serialize)]
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
    pub social_account: Option<Vec<String>>, // Using Vec<String> for TEXT[] type
    pub created_at: NaiveDateTime, // Using NaiveDateTime for TIMESTAMP
    pub updated_at: NaiveDateTime, // Using NaiveDateTime for TIMESTAMP
}


pub async fn select(client: &PgClient, username: &str) -> Result<Vec<Account>, ServiceError> {
    let stmt = r#"
        SELECT
            *
        FROM
            account
        WHERE username = $1;
    "#;
    // If no qualified user, raise a BizError of DataNotFound
    let rows = client
        .query(stmt, &[&username])
        .await?;

    rows.iter()
        .map(|row| Account::from_row_ref(&row).map_err(Into::into))
        .collect::<Result<Vec<Account>, ServiceError>>()
}

pub async fn add_account(pc: &PgClient, username: &str, password: &str) -> Result<Account, ServiceError> {
    let hashed_pwd = hash(password, DEFAULT_COST)?;

    let stmt = "INSERT INTO account(username, password) VALUES ($1, $2) RETURNING *";

    let row = pc
        .query_one(stmt, &[&username.to_string(), &hashed_pwd])
        .await?;

    Account::from_row_ref(&row).map_err(|e| {
        debug!("from row ref: {:?}", e);
        Into::into(e)
    })
}