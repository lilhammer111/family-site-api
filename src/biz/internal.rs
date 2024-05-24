use actix_web::{HttpMessage, HttpRequest, web};
use crate::AppState;
use crate::infra::error::biz::BizKind::ClaimsNotFound;
use crate::infra::error::error::Kind::{BizError, InfraError};
use crate::infra::error::error::ServiceError;
use crate::infra::middleware::jwt::Claims;
use deadpool_postgres::{Client as PgClient};

pub const MAX_PAGE_SIZE: i64 = 20;
pub const MIN_PAGE_SIZE: i64 = 10;

pub fn extract_user_id(req: HttpRequest) -> Result<i64, ServiceError> {
    let user_id = req.extensions()
        .get::<Claims>()
        .ok_or_else(|| {
            ServiceError::build()
                .belong(BizError(ClaimsNotFound))
                .done()
        })?
        .sub;

    Ok(user_id)
}

pub async fn get_pg(app_state: &web::Data<AppState>) -> Result<PgClient, ServiceError> {
    app_state.pool
        .get()
        .await
        .map_err(|err| {
            ServiceError::build()
                .belong(InfraError)
                .because(Box::new(err))
                .done()
        })
}
