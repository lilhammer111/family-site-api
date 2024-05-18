use actix_web::{HttpResponse, ResponseError};
use deadpool_postgres::PoolError;
use derive_more::{Display};
use tokio_postgres::error::SqlState;

#[derive(Display, Debug)]
pub enum BizError {
    JwtError,
    NotFound,
    CreatedError,
    PgError(tokio_postgres::Error),
    PgMapperError(tokio_pg_mapper::Error),
    PoolError(PoolError),
    BcryptError(bcrypt::BcryptError),
}

impl From<tokio_postgres::Error> for BizError {
    fn from(err: tokio_postgres::Error) -> Self {
        // 判断错误类型是否为 "No Data Found"
        if let Some(code) = err.code() {
            if code == &SqlState::NO_DATA_FOUND {
                return BizError::NotFound;
            }
        }
        BizError::PgError(err)
    }
}

impl From<tokio_pg_mapper::Error> for BizError {
    fn from(err: tokio_pg_mapper::Error) -> Self {
        //
        BizError::PgMapperError(err)
    }
}

impl From<bcrypt::BcryptError> for BizError {
    fn from(err: bcrypt::BcryptError) -> Self {
        BizError::BcryptError(err)
    }
}

impl std::error::Error for BizError {}

impl ResponseError for BizError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            BizError::NotFound => HttpResponse::NotFound().finish(),
            _ => HttpResponse::InternalServerError().finish(),
        }
    }
}

