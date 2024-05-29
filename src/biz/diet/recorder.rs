use chrono::{NaiveDate};
use deadpool_postgres::Client as PgClient;
use log::debug;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::Client;
use crate::biz::diet::courier::DietJson;
use crate::infra::error::biz::BizKind::DataNotFound;
use crate::infra::error::error::ServiceError;
use crate::infra::error::error::Kind::BizError;

#[derive(Deserialize, PostgresMapper, Debug, Serialize, Default)]
#[pg_mapper(table = "Diet")]
pub struct DietRecord {
    pub id: i64,
    pub milk: i32,
    pub meat: i32,
    pub egg: i32,
    pub vegetable: i32,
    pub fruit: i32,
    pub grain: i32,
    pub record_date: NaiveDate,
}


pub(crate) async fn insert(pg_client: &PgClient, diet_body: &DietJson) -> Result<DietRecord, ServiceError> {
    let stmt = r#"
        INSERT INTO
            diet (
                milk,
                meat,
                egg,
                vegetable,
                fruit,
                grain,
                record_date
            )
        VALUES
            ($1, $2, $3, $4, $5, $6, $7)
        RETURNING *;
    "#;

    let row = pg_client
        .query_one(
            stmt,
            &[
                &diet_body.milk,
                &diet_body.meat,
                &diet_body.egg,
                &diet_body.vegetable,
                &diet_body.fruit,
                &diet_body.grain,
                &diet_body.record_date,
            ],
        )
        .await?;

    let diet_record = DietRecord::from_row_ref(&row)?;

    Ok(diet_record)
}

pub async fn select_all(client: &Client) -> Result<Vec<DietRecord>, ServiceError> {
    let stmt = r#"
        SELECT
            *
        FROM
            diet
        ORDER BY
            record_date DESC
    "#;



    let rows = client
        .query(stmt, &[])
        .await?;

    return if rows.is_empty() {
        Err(
            ServiceError::build()
                .belong(BizError(DataNotFound))
                .done()
        )
    } else {
        let mut diet_records = Vec::new();

        for row in rows {
            let diet_record = DietRecord::from_row_ref(&row)?;
            diet_records.push(diet_record)
        }

        Ok(diet_records)
    };
}

pub(crate) async fn select_many(pc: &PgClient, page_number: i64, page_size: i64) -> Result<Vec<DietRecord>, ServiceError> {
    debug!("page number: {}, page size: {}",page_number, page_size);

    let stmt = r#"
        SELECT
            *
        FROM
            diet
        ORDER BY
            created_at DESC
        LIMIT
            $2
        OFFSET
            $1;
    "#;


    let offset = page_number * page_size;

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
        let mut diet = Vec::new();

        for row in rows {
            let diet_record = DietRecord::from_row_ref(&row)?;
            diet.push(diet_record)
        }

        Ok(diet)
    };
}

pub(crate) async fn count(pc: &PgClient) -> Result<i64, ServiceError> {
    let stmt = r#"SELECT COUNT(*) FROM diet"#;

    let count = pc.query_one(stmt, &[])
        .await?
        .get(0);

    Ok(count)
}