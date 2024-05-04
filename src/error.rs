use actix_web::{HttpResponse, ResponseError};
use deadpool_postgres::PoolError;
use derive_more::{Display, From};
use tokio_pg_mapper::Error as PGMError;
use tokio_postgres::error::Error as PGError;

#[derive(Display, From, Debug)]
pub enum DbError {
    NotFound,
    CreatedError,
    PGError(PGError),
    PGMError(PGMError),
    PoolError(PoolError),
}
impl std::error::Error for DbError {}

impl ResponseError for DbError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            DbError::NotFound => HttpResponse::NotFound().finish(),
            DbError::PoolError(ref err) => {
                HttpResponse::InternalServerError().body(err.to_string())
            }
            _ => HttpResponse::InternalServerError().finish(),
        }
    }
}