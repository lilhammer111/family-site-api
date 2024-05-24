use deadpool_postgres::{Client as PgClient, Client, GenericClient};
use tokio_pg_mapper::{Error, FromTokioPostgresRow};
use crate::biz::account::recorder::Account;
use crate::infra::error::error::ServiceError;

pub type UserRecorder = Account;

pub async fn update_account<T: Into<UserRecorder>>(pc: &PgClient, user_id: i64, t: T) -> Result<UserRecorder, ServiceError> {
    let user = t.into();

    let stmt = r#"
        UPDATE account
        SET
            username = $2,
            mobile = $3,
            email = $4,
            avatar_url = $5,
            pronouns = $6,
            birthday = $7,
            industry = $8,
            location = $9,
            social_account = $10,
            updated_at = NOW()
        WHERE id = $1
        RETURNING *;
    "#;
    let row = pc
        .query_one(stmt, &[
            &user_id,
            &user.username,
            &user.mobile,
            &user.email,
            &user.avatar_url,
            &user.pronouns,
            &user.birthday,
            &user.industry,
            &user.location,
            &user.social_account,
        ])
        .await?;

    let account_record = Account::from_row_ref(&row)?;

    Ok(account_record)
}

pub async fn query_account_by_id(pc: &PgClient, user_id: i64) -> Result<UserRecorder, ServiceError> {
    let stmt = r#"
        SELECT
            *
        FROM
            account
        WHERE id = $1;
    "#;

    let row = pc
        .query_one(stmt, &[&user_id, ])
        .await?;

    let account_record = Account::from_row_ref(&row)?;

    Ok(account_record)
}

pub async fn select_many(client: &Client, user_ids: &[i64]) -> Result<Vec<UserRecorder>, ServiceError> {
    let stmt = r#"
        SELECT
            *
        FROM
            account
        WHERE
            id = ANY($1)
    "#;

    let rows = client.query(stmt, &[&user_ids]).await?;

    Ok(
        rows.into_iter()
            .map(|row| UserRecorder::from_row_ref(&row))
            .collect::<Result<Vec<UserRecorder>, Error>>()?
    )
}