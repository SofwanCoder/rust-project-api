use crate::api::RequestId;
use std::future::{ready, Ready};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::header::{HeaderName, HeaderValue},
    Error,
    HttpMessage,
};
use futures_util::future::LocalBoxFuture;
use tracing::{trace, Instrument};

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
    type Error = Error;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;
    type InitError = ();
    type Response = ServiceResponse<B>;
    type Transform = AppRequestMiddleware<S>;

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
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;
    type Response = ServiceResponse<B>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let request_id = RequestId::default();
        let span = tracing::span!(
            tracing::Level::INFO,
            "ApiRequestMiddleware::call",
            request_id = request_id.to_string()
        )
        .entered();
        trace!("Handling request with id: {:?}", request_id);
        req.extensions_mut().insert(request_id.clone());

        let fut = self.service.call(req);

        // Moving to async context so exit the span
        // And instrument the async function with it
        // SEE: https://docs.rs/tracing/latest/tracing/span/struct.Span.html#in-asynchronous-code
        let span = span.exit();
        return Box::pin(
            async move {
                let mut res = fut.await?;

                res.headers_mut().insert(
                    HeaderName::from_static("x-request-id"),
                    HeaderValue::from_str(&request_id.to_string()).unwrap(),
                );

                trace!("Handled request with id: {:?}", request_id);
                Ok(res)
            }
            .instrument(span),
        );
    }
}
