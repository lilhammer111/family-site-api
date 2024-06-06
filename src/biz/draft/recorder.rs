use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::{Client};
use crate::infra::error::error::ServiceError;

#[derive(Deserialize, PostgresMapper, Debug, Serialize, Default)]
#[pg_mapper(table = "Draft")]
pub struct DraftRecord {
    pub user_id: i64,
    pub text: String,
    pub updated_at: NaiveDateTime,
}

pub async fn insert(client: &Client, user_id: i64, draft: String) -> Result<(), ServiceError> {
    let stmt = r#"
        INSERT INTO
            draft (
                user_id,
                text
            )
        VALUES
            ($1, $2)
        ON CONFLICT (user_id)
        DO UPDATE SET
            text = EXCLUDED.text, updated_at = CURRENT_TIMESTAMP;
    "#;

    client
        .execute(
            stmt,
            &[
                &user_id,
                &draft
            ],
        )
        .await?;

    Ok(())
}

pub async fn select(client: &Client, user_id: i64) -> Result<Vec<DraftRecord>, ServiceError> {
    let stmt = r#"
        SELECT
            *
        FROM
            draft
        WHERE
            user_id = $1
    "#;

    let mut draft_records = Vec::new();

    let rows = client
        .query(stmt, &[&user_id])
        .await?;

    for row in rows {
        let dr = DraftRecord::from_row_ref(&row)?;

        draft_records.push(dr)
    }

    Ok(draft_records)
}
