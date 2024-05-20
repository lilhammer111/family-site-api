use bcrypt::{DEFAULT_COST, hash};
use deadpool_postgres::{Client as PgClient, GenericClient};
use tokio_pg_mapper::FromTokioPostgresRow;
use crate::biz::account::recorder::Account;
use crate::infra::error::biz_err::BizError;



pub async fn update_account(pc: &PgClient, account: &Account) -> Result<Account, BizError> {
    let hashed_pwd = hash(&account.password, DEFAULT_COST)?;

    let stmt = r#"
        UPDATE account
        SET
            username = $2,
            password = $3,
            mobile = $4,
            email = $5,
            avatar_url = $6,
            pronouns = $7,
            birthday = $8,
            industry = $9,
            location = $10,
            social_account = $11,
            update_at = NOW()
        WHERE id = $1
        RETURNING *;
    "#;

    let row = pc
        .query_one(stmt, &[
            &account.id, // id
            &account.username, // username
            &hashed_pwd, // password
            &account.mobile, // mobile
            &account.email, // email
            &account.avatar_url, // avatar_url
            &account.pronouns, // pronouns
            &account.birthday, // birthday
            &account.industry, // industry
            &account.location, // location
            &account.social_account, // social_account (ensure this is serialized appropriately)
        ])
        .await?;

    Account::from_row_ref(&row).map_err(Into::into)
}