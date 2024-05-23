use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::net::IpAddr;
use actix_web::{HttpResponse, ResponseError};
use chrono::NaiveDateTime;
use deadpool_postgres::PoolError;
use derive_more::Display;
use env_logger::fmt::Timestamp;
use tokio_postgres::error::SqlState;
use crate::biz::base_comm::{Communicator, Empty};
use crate::infra::error::biz::{BizError, BizKind};
use crate::infra::error::infra::InfraError;

#[derive(Debug, PartialEq, Default)]
pub enum Kind {
    BizError(BizKind),
    #[default]
    InfraError,
}

#[derive(Debug, Display)]
pub struct ServerError {
    kind: Kind,
    because: Box<dyn Error>,
}

impl ServerError {
    pub fn build() -> ServerErrorBuilder {
        ServerErrorBuilder::default()
    }

    /// The kind method returns one of **Biz**, **Infra** kind.
    pub fn kind(&self) -> &Kind {
        &self.kind
    }

    pub fn because(&self) -> &Box<dyn Error> {
        &self.because
    }

    pub fn is_biz_err(&self) -> bool {
        return matches!(&self.kind, Kind::BizError(_));
    }

    pub fn biz_err_kind(&self) -> Option<BizKind> {
        if let Kind::BizError(biz_kind) = &self.kind() {
            Some(*biz_kind)
        } else {
            None
        }
    }
}

#[derive(Default)]
struct ServerErrorBuilder {
    kind: Kind,
    because: Box<dyn Error>,
}

impl ServerErrorBuilder {
    pub fn belong(self, kind: Kind) -> Self {
        Self {
            kind,
            ..self
        }
    }

    pub fn because(self, because: Box<dyn Error>) -> Self {
        Self {
            because,
            ..self
        }
    }


    pub fn done(self) -> ServerError {
        ServerError {
            kind: self.kind,
            because: self.because,
        }
    }
}

impl From<tokio_postgres::Error> for ServerError {
    fn from(err: tokio_postgres::Error) -> Self {
        // 判断错误类型是否为 "No Data Found"
        if let Some(code) = err.code() {
            if code == &SqlState::NO_DATA_FOUND {
                return ServerError {
                    kind: Kind::BizError(BizKind::DataNotFound),
                    because: Box::new(
                        BizError::build()
                            .message("Data not found in db")
                            .done()
                    ),
                };
            }
        }
        ServerError {
            kind: Kind::InfraError,
            because: Box::new(err),
        }
    }
}

impl From<tokio_pg_mapper::Error> for ServerError {
    fn from(err: tokio_pg_mapper::Error) -> Self {
        ServerError {
            kind: Kind::InfraError,
            because: Box::new(err),
        }
    }
}


impl Error for ServerError {}

impl Display for ServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "kind: {:?}, cause: {}", self.kind, self.because)
    }
}

impl ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        match self.biz_err_kind() {
            // if none, it indicates that the error instance is not type of business error.
            None => {
                HttpResponse::InternalServerError().json(
                    Communicator::<Empty>::builder()
                        .message("Internal server error")
                        .build()
                )
            }
            // otherwise business error
            Some(biz_err_kind) => {
                match biz_err_kind {
                    BizKind::Other => {
                        HttpResponse::InternalServerError().json(
                            Communicator::<Empty>::builder()
                                .message("Internal server error")
                                .build()
                        )
                    }
                    BizKind::DataNotFound => {
                        HttpResponse::NotFound().json(
                            Communicator::<Empty>::builder()
                                .message("Data is not found")
                                .build()
                        )
                    }
                }
            }
        }
    }
}

impl From<bcrypt::BcryptError> for ServerError {
    fn from(err: bcrypt::BcryptError) -> Self {
        ServerError {
            kind: Kind::InfraError,
            because: Box::new(err),
        }
    }
}