use actix_web::{dev::{Service, ServiceRequest, ServiceResponse, Transform}, Error, HttpMessage, HttpResponse};
use actix_web::dev::forward_ready;
use futures_util::{future::{Ready, ready, LocalBoxFuture}};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use crate::infra::config::Settings;

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: i64, // 用户标识
    pub exp: i64,  // 过期时间
}

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

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let secret_key = if let Some(data) = req.app_data::<Settings>() {
            data.jwt_secret.clone()
        } else {
            return Box::pin(
                async {
                    Err(Error::from(HttpResponse::InternalServerError().finish()))
                }
            );
        };


        let token = if let Some(auth_header) = req.headers().get("Authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    Some(auth_str[7..].to_string())
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        if let Some(token) = token {
            match decode::<Claims>(
                &token,
                &DecodingKey::from_secret(secret_key.as_ref()),
                &Validation::new(Algorithm::HS256),
            ) {
                Ok(data) => {
                    req.extensions_mut().insert(data.claims);
                }
                Err(_) => {
                    return Box::pin(async { Err(Error::from(HttpResponse::Unauthorized().finish())) });
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
