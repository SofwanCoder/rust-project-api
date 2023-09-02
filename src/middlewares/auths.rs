use crate::helpers::response;
use crate::helpers::response::AppResponse;
use actix_web::http::StatusCode;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures_util::future::{Either, LocalBoxFuture};
use std::future::{ready, Ready};

pub struct Authorization;

impl Default for Authorization {
    fn default() -> Self {
        Authorization {}
    }
}

impl<S> Transform<S, ServiceRequest> for Authorization
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Transform = AuthorizationMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;
    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthorizationMiddleware { service }))
    }
}
pub struct AuthorizationMiddleware<S> {
    service: S,
}
impl<S> Service<ServiceRequest> for AuthorizationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        enum WhatHappened {
            Nothing,
            Unauthorized,
            NotBearer,
            Malformed,
        }

        fn act(req: &ServiceRequest) -> WhatHappened {
            let authorization_value = req.headers().get("Authorization");
            if authorization_value.is_none() {
                return WhatHappened::Nothing;
            }

            // split the authorization value
            let authorization_value_split = authorization_value
                .unwrap()
                .to_str()
                .unwrap()
                .split(" ")
                .collect::<Vec<&str>>();

            if authorization_value_split[0].ne("Bearer") {
                return WhatHappened::NotBearer;
            }

            if authorization_value_split.len() != 2 {
                return WhatHappened::Malformed;
            }

            let authorization_token = authorization_value_split[1];

            let decoded_jwt = crate::helpers::jwt::decode::<crate::types::auths::AuthenticatedData>(
                authorization_token,
            );

            if decoded_jwt.is_err() {
                return WhatHappened::Unauthorized;
            }

            let auth_data = decoded_jwt.unwrap();

            req.extensions_mut().insert(auth_data);

            return WhatHappened::Nothing;
        }

        let what = act(&req);

        let either = match what {
            WhatHappened::NotBearer => {
                let r = response::app_http_response(
                    StatusCode::IM_A_TEAPOT,
                    AppResponse::<()> {
                        message: "Bearer token expected".to_string(),
                        data: None,
                        errors: None,
                    },
                );
                Either::Left(req.into_response(r))
            }
            _ => Either::Right(self.service.call(req)),
        };

        return Box::pin(async move {
            return match either {
                Either::Left(res) => Ok(res),
                Either::Right(fut) => fut.await,
            };
        });
    }
}
