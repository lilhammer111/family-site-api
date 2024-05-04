use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use deadpool_postgres::Client as PgClient;
use tokio_pg_mapper::FromTokioPostgresRow;
use crate::error::DbError;

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "account")]
pub struct Account {
    pub account_name: String,
    pub password: String,
}

pub async fn get_account_pwd(pc: &PgClient, account_name: &str) -> Result<String, DbError> {
    let stmt = "SELECT password FROM account WHERE account_name = $1";

    pc
        .query(stmt, &[&account_name])
        .await?
        .iter()
        .next()
        .ok_or(DbError::NotFound)
        .map(|row| row.get("password"))
}

pub async fn add_account(pc: &PgClient, account: Account) -> Result<Account, DbError> {
    let stmt = "INSERT INTO account(account_name, password) VALUES ($1, $2) RETURNING *";

    pc
        .query(
            stmt,
            &[
                &account.account_name,
                &account.password,
            ],
        )
        .await?
        .iter()
        .next()
        .ok_or(DbError::CreatedError)
        .map(|row| Account::from_row_ref(row).expect("Failed to mapping the record to struct."))
}