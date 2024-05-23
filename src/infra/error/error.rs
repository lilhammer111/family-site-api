use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use actix_web::{HttpResponse, ResponseError};
use chrono::NaiveDateTime;
use jsonwebtoken::errors::ErrorKind;
use tokio_postgres::error::SqlState;
use crate::biz::base_comm::{Communicator, Empty};
use crate::infra::error::biz::BizKind;
use crate::infra::error::biz::BizKind::{DataNotFound, TokenInvalid};
use crate::infra::error::error::Kind::{BizError, InfraError};

#[derive(Debug, PartialEq, Default)]
pub enum Kind {
    BizError(BizKind),
    #[default]
    InfraError,
}

#[derive(Debug)]
pub struct ServiceError {
    kind: Kind,
    who: Option<i64>,
    when: NaiveDateTime,
    because: Box<dyn Error>,
    message: String,
}

#[derive(Debug)]
struct UnknownError;

impl Display for UnknownError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "unknown")
    }
}

impl Error for UnknownError {}

impl Display for ServiceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "user: {:?}, at: {}, kind: {:?}, cause: {:?}, extra: {}",
               self.who(),
               self.when(),
               self.kind(),
               self.because(),
               self.message()
        )
    }
}

impl Error for ServiceError {}

impl ServiceError {
    pub fn build() -> ServerErrorBuilder {
        ServerErrorBuilder::default()
    }

    /// The kind method returns one of **Biz**, **Infra** kind.
    pub fn kind(&self) -> &Kind {
        &self.kind
    }

    pub fn who(&self) -> &Option<i64> {
        &self.who
    }

    pub fn when(&self) -> &NaiveDateTime {
        &self.when
    }

    pub fn because(&self) -> &Box<dyn Error> {
        &self.because
    }

    pub fn message(&self) -> &String {
        &self.message
    }

    /// If err belongs to `Kind::Biz` error, return the concrete kind,
    /// or return `None`
    pub fn biz_kind(&self) -> Option<BizKind> {
        if let BizError(biz_kind) = &self.kind() {
            Some(*biz_kind)
        } else {
            None
        }
    }
}


#[derive(Debug)]
pub struct ServerErrorBuilder {
    kind: Kind,
    because: Box<dyn Error>,
    who: Option<i64>,
    when: NaiveDateTime,
    message: String,
}

impl Display for ServerErrorBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "user: {:?}, at: {}, kind: {:?}, cause: {:?}, extra: {}",
               self.who,
               self.when,
               self.kind,
               self.because,
               self.message
        )
    }
}

impl Default for ServerErrorBuilder {
    fn default() -> Self {
        Self {
            kind: Default::default(),
            because: Box::new(UnknownError {}),
            who: None,
            when: Default::default(),
            message: "".to_string(),
        }
    }
}


impl ServerErrorBuilder {
    pub fn belong(self, kind: Kind) -> Self {
        Self {
            kind,
            ..self
        }
    }

    pub fn because(self, err: Box<dyn Error>) -> Self {
        Self {
            because: err,
            ..self
        }
    }

    // pub fn who(self, who: i64) -> Self {
    //     Self {
    //         who: Some(who),
    //         ..self
    //     }
    // }

    // pub fn message(self, message: &str) -> Self {
    //     Self {
    //         message: message.to_string(),
    //         ..self
    //     }
    // }


    pub fn done(self) -> ServiceError {
        ServiceError {
            kind: self.kind,
            because: self.because,
            who: self.who,
            when: Default::default(),
            message: self.message,
        }
    }
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self.biz_kind() {
            // if none, it indicates that the error instance is not type of business error.
            None => {
                HttpResponse::InternalServerError().json(
                    Communicator::<Empty>::build()
                        .message("Internal server error")
                        .done()
                )
            }
            // otherwise business error
            Some(biz_err_kind) => {
                match biz_err_kind {
                    DataNotFound => {
                        HttpResponse::NotFound().json(
                            Communicator::<Empty>::build()
                                .message("Data is not found")
                                .done()
                        )
                    }
                    _ => {
                        HttpResponse::InternalServerError().json(
                            Communicator::<Empty>::build()
                                .message("Internal server error")
                                .done()
                        )
                    }
                }
            }
        }
    }
}

impl From<bcrypt::BcryptError> for ServiceError {
    fn from(err: bcrypt::BcryptError) -> Self {
        ServiceError::build()
            .belong(InfraError)
            .because(Box::new(err))
            .done()
    }
}

impl From<tokio_postgres::Error> for ServiceError {
    fn from(err: tokio_postgres::Error) -> Self {
        // 判断错误类型是否为 "No Data Found"
        if let Some(code) = err.code() {
            if code == &SqlState::NO_DATA_FOUND {
                return ServiceError::build()
                    .belong(BizError(DataNotFound))
                    .because(Box::new(err))
                    .done();
            }
        }

        ServiceError::build()
            .belong(InfraError)
            .because(Box::new(err))
            .done()
    }
}

impl From<tokio_pg_mapper::Error> for ServiceError {
    fn from(err: tokio_pg_mapper::Error) -> Self {
        ServiceError::build()
            .belong(InfraError)
            .because(Box::new(err))
            .done()
    }
}

impl From<jsonwebtoken::errors::Error> for ServiceError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        match err.kind() {
            ErrorKind::InvalidToken | ErrorKind::ExpiredSignature => {
                ServiceError::build()
                    .belong(BizError(TokenInvalid))
                    .because(Box::new(err))
                    .done()
            }
            _ => {
                ServiceError::build()
                    .belong(InfraError)
                    .because(Box::new(err))
                    .done()
            }
        }
    }
}