use std::fmt::{Debug, Display};
use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;

#[derive(Display, Debug)]
pub enum InfraError<D> {
    DepError(D),
}


impl<D> From<D> for InfraError<D> {
    fn from(dep: D) -> Self {
        InfraError::DepError(dep)
    }
}

impl<D: Debug + Display> std::error::Error for InfraError<D> {}

impl<D: Debug + Display> ResponseError for InfraError<D> {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::InternalServerError().finish()
    }
}

