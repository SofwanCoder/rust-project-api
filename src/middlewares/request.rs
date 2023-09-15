use crate::RequestId;
use std::future::{ready, Ready};

use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures_util::future::LocalBoxFuture;

pub struct AppRequest;

impl Default for AppRequest {
    fn default() -> Self {
        AppRequest
    }
}

impl<S, B> Transform<S, ServiceRequest> for AppRequest
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AppRequestMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AppRequestMiddleware { service }))
    }
}

pub struct AppRequestMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AppRequestMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let request_id = RequestId::default();
        req.extensions_mut().insert(request_id.clone());

        let fut = self.service.call(req);

        return Box::pin(async move {
            let mut res = fut.await?;

            res.headers_mut().insert(
                HeaderName::from_static("x-request-id"),
                HeaderValue::from_str(&request_id.id.to_string()).unwrap(),
            );

            Ok(res)
        });
    }
}
