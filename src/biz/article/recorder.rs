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
pub struct ArticleRecord {
    pub id: i64,
    pub kind: String,
    pub tags: Vec<Option<String>>,
    pub is_trending: Option<bool>,
    pub is_insight: Option<bool>,
    pub is_recommend: Option<bool>,
    pub cover_url: Option<String>,
    pub title: String,
    pub summary: Option<String>,
    pub text: Option<String>,
    pub text_url: Option<String>,
    pub author_id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}


pub(crate) async fn insert(client: &Client, article_courier: courier::ArticleCourier, author_id: i64) -> Result<ArticleRecord, ServiceError> {
    let stmt = r#"
        INSERT INTO
            article (
                kind,
                tags,
                is_trending,
                is_insight,
                is_recommend,
                cover_url,
                title,
                summary,
                text,
                text_url,
                author_id
            )
        VALUES
            ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        RETURNING *;
    "#;

    let row = client
        .query_one(
            stmt,
            &[
                &article_courier.kind,
                &article_courier.tags,
                &article_courier.is_trending,
                &article_courier.is_insight,
                &article_courier.is_recommend,
                &article_courier.cover_url,
                &article_courier.title,
                &article_courier.summary,
                &article_courier.text,
                &article_courier.text_url,
                &author_id
            ],
        )
        .await?;

    let article_record = ArticleRecord::from_row_ref(&row)?;

    Ok(article_record)
}

pub async fn select_by_author_id(client: &Client, user_id: i64) -> Result<Vec<ArticleRecord>, ServiceError> {
    let stmt = r#"
        SELECT
            *
        FROM
            article
        WHERE
            author_id = $1
        ORDER BY
            created_at DESC
    "#;

    let rows = client
        .query(stmt, &[&user_id])
        .await?;

    return if rows.is_empty() {
        Err(
            ServiceError::build()
                .belong(BizError(DataNotFound))
                .message("The user do not have any article yet")
                .done()
        )
    } else {
        let mut article_records = Vec::new();

        for row in rows {
            let article_record = ArticleRecord::from_row_ref(&row)?;
            article_records.push(article_record)
        }

        Ok(article_records)
    };
}

pub async fn select_all(client: &Client) -> Result<Vec<ArticleRecord>, ServiceError> {
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
            let article_record = ArticleRecord::from_row_ref(&row)?;
            article_records.push(article_record)
        }

        Ok(article_records)
    };
}


pub(crate) async fn select_paginated(client: &Client, page_number: i64, page_size: i64) -> Result<Vec<ArticleRecord>, ServiceError> {
    debug!("page number: {}, page size: {}",page_number, page_size);

    let stmt = r#"
        SELECT
            *
        FROM
            article
        ORDER BY
            created_at DESC
        LIMIT
            $1
        OFFSET
            $2;
    "#;


    let offset = page_number * page_size;

    let rows = client
        .query(stmt, &[&page_size, &offset])
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
            let article_record = ArticleRecord::from_row_ref(&row)?;
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

