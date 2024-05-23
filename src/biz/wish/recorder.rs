use chrono::NaiveDateTime;
use deadpool_postgres::{Client as PgClient, GenericClient};
use serde::{Deserialize, Serialize};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::error::Error as PgError;
use crate::infra::error::biz_err::BizError;

#[derive(Deserialize, PostgresMapper, Debug, Serialize)]
#[pg_mapper(table = "account")]
pub struct WishRecord {
    pub id: i64,
    pub user_id: i64,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub async fn insert(pg_client: &PgClient, user_id: i64, content: &str) -> Result<WishRecord, BizError> {
    let stmt = r#"
        INSERT INTO wish(user_id, content)
        VALUES ($1, $2)
        RETURNING *;
    "#;

    let row = pg_client
        .query_one(stmt, &[&user_id, &content])
        .await?;

    let wish = WishRecord::from_row_ref(&row)?;

    Ok(wish)
}


pub async fn select_many(pc: &PgClient, page_number: i64, page_size: i64) -> Result<Vec<WishRecord>, BizError> {
    let stmt = r#"
        SELECT
            id,
            user_id,
            content,
            created_at
        FROM
            wish
        ORDER_BY
            created_at DESC
        LIMIT
            $2
        OFFSET
            ($1 - 1) * $2;
    "#;

    let rows = pc
        .query(stmt, &[&page_number, &page_size])
        .await?;

    return if rows.is_empty() {
        Err(BizError::NotFound)
    } else {
        let mut wishes = Vec::new();

        for row in rows {
            let wish_record = WishRecord::from_row_ref(&row)?;
            wishes.push(wish_record)
        }

        Ok(wishes)
    };
}

pub async fn count(pc: &PgClient) -> Result<i64, PgError> {
    let stmt = r#"SELECT COUNT(*) FROM wish"#;

    let count = pc.query_one(stmt, &[])
        .await?
        .get(0);

    Ok(count)
}