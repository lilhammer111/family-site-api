use deadpool_postgres::{Client as PgClient, GenericClient};
use log::debug;
use tokio_pg_mapper::FromTokioPostgresRow;
use crate::biz::account::recorder::Account;
use crate::infra::error::biz_err::BizError;

pub type UserRecorder = Account;

pub async fn update_account<T: Into<UserRecorder>>(pc: &PgClient,user_id: i64, t: T) -> Result<UserRecorder, BizError> {
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
        .await
        .map_err(|err| {
            if err.to_string() == crate::biz::account::recorder::QUERY_MORE_THAN_ONE {
                BizError::NotFound
            } else {
                BizError::PgError(err)
            }
        })?;

    Account::from_row_ref(&row).map_err(|e| {
        debug!("from row ref: {:?}", e);
        Into::into(e)
    })
}


pub async fn query_account_by_id(pc: &PgClient, user_id: i64) -> Result<UserRecorder, BizError> {
    let stmt = r#"
        SELECT
            *
        FROM
            account
        WHERE id = $1;
    "#;

    let row = pc
        .query_one(stmt, &[&user_id, ])
        .await
        .map_err(|err| {
            if err.to_string() == crate::biz::account::recorder::QUERY_MORE_THAN_ONE {
                BizError::NotFound
            } else {
                BizError::PgError(err)
            }
        })?;

    Account::from_row_ref(&row).map_err(|e| {
        debug!("from row ref: {:?}", e);
        Into::into(e)
    })
}
