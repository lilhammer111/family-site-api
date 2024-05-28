use chrono::{NaiveDate, NaiveTime};
use deadpool_postgres::Client as PgClient;
use log::debug;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;
use crate::biz::behavior::courier::Behavior;
use crate::infra::error::biz::BizKind::DataNotFound;
use crate::infra::error::error::ServiceError;
use crate::infra::error::error::Kind::BizError;

#[derive(Deserialize, PostgresMapper, Debug, Serialize, Default)]
#[pg_mapper(table = "Behavior")]
pub struct BehaviorRecord {
    pub id: i64,
    pub wake_up_time: NaiveTime,
    pub sleep_time: NaiveTime,
    pub diaper_changes: i32,
    pub naps: i32,
    pub crying_episodes: i32,
    pub outdoor_time: i32,
    pub record_date: NaiveDate,
}

pub(crate) async fn insert(pg_client: &PgClient, behavior_json: &Behavior) -> Result<BehaviorRecord, ServiceError> {
    let stmt = r#"
        INSERT INTO
            behavior (
               wake_up_time,
               sleep_time,
               diaper_changes,
               naps,
               crying_episodes,
               outdoor_time,
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
                &behavior_json.wake_up_time,
                &behavior_json.sleep_time,
                &behavior_json.diaper_changes,
                &behavior_json.naps,
                &behavior_json.crying_episodes,
                &behavior_json.outdoor_time,
                &behavior_json.record_date,
            ],
        )
        .await?;

    let behavior_record = BehaviorRecord::from_row_ref(&row)?;

    Ok(behavior_record)
}


pub(crate) async fn select_many(pc: &PgClient, page_number: i64, page_size: i64) -> Result<Vec<BehaviorRecord>, ServiceError> {
    debug!("page number: {}, page size: {}",page_number, page_size);

    let stmt = r#"
        SELECT
            *
        FROM
            behavior
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
        let mut behavior = Vec::new();

        for row in rows {
            let behavior_record = BehaviorRecord::from_row_ref(&row)?;
            behavior.push(behavior_record)
        }

        Ok(behavior)
    };
}

pub(crate) async fn count(pc: &PgClient) -> Result<i64, ServiceError> {
    let stmt = r#"SELECT COUNT(*) FROM behavior"#;

    let count = pc.query_one(stmt, &[])
        .await?
        .get(0);

    Ok(count)
}