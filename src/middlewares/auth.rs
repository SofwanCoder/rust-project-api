use crate::helpers::response::AppResponse;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::StatusCode,
    Error,
    HttpMessage,
};
use futures_util::future::{Either, LocalBoxFuture};
use std::future::{ready, Ready};
use tracing::{error, instrument, trace, warn, Instrument};

pub struct Authorization;

impl Default for Authorization {
    fn default() -> Self {
        Authorization
    }
}

impl<S> Transform<S, ServiceRequest> for Authorization
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static,
{
    type Error = S::Error;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;
    type InitError = ();
    type Response = S::Response;
    type Transform = AuthorizationMiddleware<S>;

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
    type Error = S::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;
    type Response = S::Response;

    forward_ready!(service);

    #[instrument(fields(middlware = "AuthorizationMiddleware::call"), skip_all)]
    fn call(&self, req: ServiceRequest) -> Self::Future {
        trace!("Processing authorization middleware");
        enum WhatHappened {
            Nothing,
            Unauthorized,
            NotBearer,
            Malformed,
        }

        let decrypt_authorization_fn = || -> WhatHappened {
            trace!("Decrypting authorization header");
            let authorization_value = req.headers().get("Authorization");
            if authorization_value.is_none() {
                trace!("Authorization header is not present");
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
                warn!(
                    "Bearer token expected; found {:?}",
                    authorization_value_split[0]
                );
                return WhatHappened::NotBearer;
            }

            if authorization_value_split.len() != 2 {
                warn!("Malformed authorization header");
                return WhatHappened::Malformed;
            }

            let authorization_token = authorization_value_split[1];

            let decoded_jwt = crate::utilities::jwt::decode::<
                crate::types::auth_types::AuthenticatedData,
            >(authorization_token);

            if decoded_jwt.is_err() {
                error!("JWT token is invalid with error: {:?}", decoded_jwt.err());
                return WhatHappened::Unauthorized;
            }

            let auth_data = decoded_jwt.unwrap();

            req.extensions_mut().insert(auth_data);

            return WhatHappened::Nothing;
        };

        let what = decrypt_authorization_fn();

        let either = match what {
            WhatHappened::NotBearer => {
                let r = AppResponse::Error::<()>("Bearer token expected", None)
                    .to_http_response(StatusCode::IM_A_TEAPOT);
                Either::Left(req.into_response(r))
            }
            _ => Either::Right(self.service.call(req)),
        };

        return Box::pin(
            async move {
                return match either {
                    Either::Left(res) => Ok(res),
                    Either::Right(fut) => fut.await,
                };
            }
            .instrument(tracing::Span::current()),
        );
    }
}
