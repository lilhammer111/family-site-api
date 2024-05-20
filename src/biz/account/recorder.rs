use bcrypt::{DEFAULT_COST, hash};
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use deadpool_postgres::{Client as PgClient, GenericClient};
use tokio_pg_mapper::FromTokioPostgresRow;
use chrono::{NaiveDate};
use chrono::NaiveDateTime;
use log::debug;
use tokio_postgres::types::ToSql;
use crate::infra::error::biz_err::BizError;

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

const QUERY_MORE_THAN_ONE: &str = "query returned an unexpected number of rows";


async fn execute_query<T>(pc: &PgClient, stmt: &str, param: T) -> Result<Account, BizError>
    where T: ToSql + Sync
{
    let row = pc
        .query_one(stmt, &[&param])
        .await
        .map_err(|err| {
            if err.to_string() == QUERY_MORE_THAN_ONE {
                BizError::NotFound
            } else {
                BizError::PgError(err)
            }
        })?;

    Account::from_row_ref(&row).map_err(|e| {
        debug!("from row ref: {:?}", e);
        Into::into(e)
    })
}

pub async fn query_account_by_id(pc: &PgClient, user_id: i64) -> Result<Account, BizError> {
    let stmt = r#"
        SELECT
            *
        FROM
            account
        WHERE id = $1;
    "#;

    execute_query(pc, stmt, user_id).await
}

pub async fn query_account(pc: &PgClient, username: &str) -> Result<Account, BizError> {
    let stmt = r#"
        SELECT
            *
        FROM
            account
        WHERE username = $1;
    "#;

    execute_query(pc, stmt, username).await
}

pub async fn add_account(pc: &PgClient, username: &str, password: &str) -> Result<Account, BizError> {
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