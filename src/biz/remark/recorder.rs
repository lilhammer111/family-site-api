use chrono::{NaiveDateTime};
use log::debug;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::Client;
use crate::biz::remark::courier;
use crate::infra::error::biz::BizKind::DataNotFound;
use crate::infra::error::error::Kind::BizError;
use crate::infra::error::error::ServiceError;

#[derive(Deserialize, PostgresMapper, Debug, Serialize, Default)]
#[pg_mapper(table = "Remark")]
pub struct RemarkRecorder {
    pub id: i64,
    pub user_id: i64,
    // main item primary key, such as id
    pub parent: i64,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}


pub(crate) async fn insert(client: &Client, remark_courier: courier::RemarkCourier, user_id: i64) -> Result<RemarkRecorder, ServiceError> {
    let stmt = r#"
        INSERT INTO
            remark (
                user_id,
                parent,
                content
            )
        VALUES
            ($1, $2, $3)
        RETURNING *;
    "#;

    let row = client
        .query_one(
            stmt,
            &[
                &user_id,
                &remark_courier.parent,
                &remark_courier.content,
            ],
        )
        .await?;

    let remark_record = RemarkRecorder::from_row_ref(&row)?;

    Ok(remark_record)
}

pub async fn select_all(client: &Client) -> Result<Vec<RemarkRecorder>, ServiceError> {
    let stmt = r#"
        SELECT
            *
        FROM
            remark
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
                .message("No remark yet")
                .done()
        )
    } else {
        let mut remark_records = Vec::new();

        for row in rows {
            let remark_record = RemarkRecorder::from_row_ref(&row)?;
            remark_records.push(remark_record)
        }

        Ok(remark_records)
    };
}


pub(crate) async fn select_paginated(client: &Client, parent: i64, page_number: i64, page_size: i64) -> Result<Vec<RemarkRecorder>, ServiceError> {
    debug!("page number: {}, page size: {}, parent id: {}",page_number, page_size, parent);

    let stmt = r#"
        SELECT
            *
        FROM
            remark
        WHERE
            parent = $3
        ORDER BY
            created_at DESC
        LIMIT
            $1
        OFFSET
            $2;
    "#;


    let offset = page_number * page_size;

    let rows = client
        .query(stmt, &[&page_size, &offset, &parent])
        .await?;

    return if rows.is_empty() {
        Err(
            ServiceError::build()
                .belong(BizError(DataNotFound))
                .done()
        )
    } else {
        let mut remark = Vec::new();

        for row in rows {
            let remark_record = RemarkRecorder::from_row_ref(&row)?;
            debug!("remark record: {:?}", remark_record);
            remark.push(remark_record)
        }

        Ok(remark)
    };
}

pub(crate) async fn count(client: &Client, parent: i64) -> Result<i64, ServiceError> {
    let stmt = r#"SELECT COUNT(*) FROM remark WHERE parent = $1"#;

    let count = client.query_one(stmt, &[&parent])
        .await?
        .get(0);

    Ok(count)
}

