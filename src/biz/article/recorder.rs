use chrono::{NaiveDateTime};
use log::debug;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::Client;
use crate::biz::article::courier;
use crate::infra::error::biz::BizKind::DataNotFound;
use crate::infra::error::error::Kind::BizError;
use crate::infra::error::error::ServiceError;

#[derive(Deserialize, PostgresMapper, Debug, Serialize, Default)]
#[pg_mapper(table = "Article")]
pub struct Article {
    pub id: i64,
    pub cover_url: String,
    pub title: String,
    pub author_id: i64,
    pub summary: Option<String>,
    pub text: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}




pub(crate) async fn insert(client: &Client, article_courier: courier::Article) -> Result<Article, ServiceError> {
    let stmt = r#"
        INSERT INTO
            article (
               cover_url,
               title,
               author_id,
               summary,
               text
            )
        VALUES
            ($1, $2, $3, $4, $5)
        RETURNING *;
    "#;

    let row = client
        .query_one(
            stmt,
            &[
                &article_courier.cover_url,
                &article_courier.title,
                &article_courier.author_id,
                &article_courier.summary,
                &article_courier.text,
            ],
        )
        .await?;

    let article_record = Article::from_row_ref(&row)?;

    Ok(article_record)
}


pub async fn select_all(client: &Client) -> Result<Vec<Article>, ServiceError> {
    let stmt = r#"
        SELECT
            *
        FROM
            article
        ORDER BY
            created_at DESC
    "#;


    let rows = client
        .query(stmt, &[])
        .await?;

    return if rows.is_empty() {
        Err(
            ServiceError::build()
                .belong(BizError(DataNotFound))
                .message("No article yet")
                .done()
        )
    } else {
        let mut article_records = Vec::new();

        for row in rows {
            let article_record = Article::from_row_ref(&row)?;
            article_records.push(article_record)
        }

        Ok(article_records)
    };
}


pub(crate) async fn select_many(client: &Client, page_number: i64, page_size: i64) -> Result<Vec<Article>, ServiceError> {
    debug!("page number: {}, page size: {}",page_number, page_size);

    let stmt = r#"
        SELECT
            *
        FROM
            article
        ORDER BY
            created_at DESC
        LIMIT
            $2
        OFFSET
            $1;
    "#;


    let offset = page_number * page_size;

    let rows = client
        .query(stmt, &[&offset, &page_size])
        .await?;

    return if rows.is_empty() {
        Err(
            ServiceError::build()
                .belong(BizError(DataNotFound))
                .done()
        )
    } else {
        let mut article = Vec::new();

        for row in rows {
            let article_record = Article::from_row_ref(&row)?;
            article.push(article_record)
        }

        Ok(article)
    };
}

pub(crate) async fn count(client: &Client) -> Result<i64, ServiceError> {
    let stmt = r#"SELECT COUNT(*) FROM article"#;

    let count = client.query_one(stmt, &[])
        .await?
        .get(0);

    Ok(count)
}

