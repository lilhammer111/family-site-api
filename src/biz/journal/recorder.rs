use chrono::NaiveDateTime;
use deadpool_postgres::Client as PgClient;
use log::debug;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;
use crate::infra::error::biz::BizKind::DataNotFound;
use crate::infra::error::error::ServiceError;
use crate::infra::error::error::Kind::BizError;

#[derive(Deserialize, PostgresMapper, Debug, Serialize)]
#[pg_mapper(table = "Journal")]
pub struct JournalRecord {
    pub id: i64,
    pub title: i64,
    pub content: String,
    pub images: Vec<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub(crate) async fn insert(pg_client: &PgClient, title: &str, content: &str, images: &[&str]) -> Result<JournalRecord, ServiceError> {
    let stmt = r#"
        INSERT INTO
            journal(title, content, images)
        VALUES ($1, $2, $3)
        RETURNING *;
    "#;

    let row = pg_client
        .query_one(stmt, &[&title, &content, &images])
        .await?;

    let journal_record = JournalRecord::from_row_ref(&row)?;

    Ok(journal_record)
}


pub(crate) async fn select_many(pc: &PgClient, page_number: i64, page_size: i64) -> Result<Vec<JournalRecord>, ServiceError> {
    debug!("page number: {}, page size: {}",page_number, page_size);

    let offset = page_number * page_size;

    let stmt = r#"
        SELECT
            id,
            user_id,
            content,
            created_at
        FROM
            wish
        ORDER BY
            created_at DESC
        LIMIT
            $2
        OFFSET
            $1;
    "#;

    let rows = pc
        .query(stmt, &[&offset, &page_size])
        .await?;

    return if rows.is_empty() {
        Err(
            ServiceError::build()
                .belong(BizError(DataNotFound))
                .done()
        )
    } else {
        let mut wishes = Vec::new();

        for row in rows {
            let wish_record = JournalRecord::from_row_ref(&row)?;
            wishes.push(wish_record)
        }

        Ok(wishes)
    };
}

pub(crate) async fn count(pc: &PgClient) -> Result<i64, ServiceError> {
    let stmt = r#"SELECT COUNT(*) FROM wish"#;

    let count = pc.query_one(stmt, &[])
        .await?
        .get(0);

    Ok(count)
}