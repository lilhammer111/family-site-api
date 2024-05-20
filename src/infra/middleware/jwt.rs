use actix_web::{dev::{Service, ServiceRequest, ServiceResponse, Transform}, Error, HttpMessage, web};
use actix_web::dev::forward_ready;
use futures_util::{future::{Ready, ready, LocalBoxFuture}};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use log::debug;
use serde::{Deserialize, Serialize};
use crate::AppState;
use crate::infra::error::biz_err::BizError;

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: i64, // user_id
    pub exp: i64,  // expires
}

pub const JWT_AUTH_KEY: &str = "auth_token";


pub struct JwtMiddleware;

impl<S, B> Transform<S, ServiceRequest> for JwtMiddleware
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = JwtMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtMiddlewareService { service }))
    }
}

pub struct JwtMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for JwtMiddlewareService<S>
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let secret_key: String;

        debug!("req: {:#?}", req);

        match req.app_data::<web::Data<AppState>>() {
            Some(data) => {
                secret_key = data.jwt_secret.clone();
                debug!("secret key: {}", secret_key)
            }
            None => {
                debug!("Failed to get data from AppState");
                return Box::pin(
                    async {
                        Err(BizError::JwtError.into())
                    }
                );
            }
        }


        let some_token = req.cookie(JWT_AUTH_KEY).map(|cookie| {
            debug!("cookie: {}",&cookie);
            cookie.value().to_string()
        });

        if let Some(token) = some_token {
            match decode::<Claims>(
                &token,
                &DecodingKey::from_secret(secret_key.as_bytes()),
                &Validation::new(Algorithm::default()),
            ) {
                Ok(data) => {
                    debug!("validation success");
                    req.extensions_mut().insert(data.claims);
                }
                Err(e) => {
                    debug!("validation failed: {:?}",e);
                    return Box::pin(
                        async { Err(BizError::JwtError.into()) }
                    );
                }
            }
        }

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
