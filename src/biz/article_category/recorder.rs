use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::Client;
use crate::infra::error::error::ServiceError;

#[derive(Deserialize, PostgresMapper, Debug, Serialize, Default)]
#[pg_mapper(table = "ArticleCategory")]
pub struct ArticleCategory {
    pub id: i32,
    pub level1: String,
    pub level2: String,
    pub level3: String,
    pub description: String,
}

pub async fn select_category(client: &Client, level3: String) -> Result<ArticleCategory, ServiceError> {
    let stmt = r#"
        SELECT
            *
        FROM
            article_category
        WHERE
            level3 = $1;
    "#;


    let row = client
        .query_one(stmt, &[&level3])
        .await?;

    let category = ArticleCategory::from_row_ref(&row)?;

    Ok(category)
}

pub async fn select_all_category(client: &Client) -> Result<Vec<ArticleCategory>, ServiceError> {
    let stmt = r#"
        SELECT
            *
        FROM
            article_category
    "#;


    let rows = client
        .query(stmt, &[])
        .await?;

    let mut categories = Vec::new();

    for row in rows {
        let category = ArticleCategory::from_row_ref(&row)?;
        categories.push(category)
    }

    Ok(categories)
}

pub async fn select_distinct_level(client: &Client) -> Result<HashMap<String, i64>, ServiceError> {
    let stmt = r#"
        SELECT
            level1, COUNT(DISTINCT level2) AS unique_level2_count
        FROM
            article_category
        GROUP BY
            level1;
    "#;


    let rows = client
        .query(stmt, &[])
        .await?;

    let mut counts = HashMap::new();

    for row in rows {
        let level1 = row.try_get("level1")?;
        let count = row.try_get("unique_level2_count")?;
        counts.insert(level1, count);
    }

    Ok(counts)
}

