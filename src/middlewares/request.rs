use crate::api::RequestId;
use std::future::{ready, Ready};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::header::{HeaderName, HeaderValue},
    Error,
    HttpMessage,
};
use futures_util::future::LocalBoxFuture;
use tracing::{instrument, trace, Instrument};

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

    #[instrument(name = "ApiRequestMiddleware::call", fields(request_id = tracing::field::Empty), skip_all)]
    fn call(&self, req: ServiceRequest) -> Self::Future {
        let request_id = RequestId::default();
        let span = tracing::Span::current();
        span.record("request_id", &request_id.to_string());
        req.extensions_mut().insert(request_id.clone());

        trace!("Handling request with id: {:?}", request_id);

        let fut = self.service.call(req);

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
                // SEE: https://docs.rs/tracing/latest/tracing/span/struct.Span.html#in-asynchronous-code
            .instrument(span), // This is the key to make the span work in async code
        );
    }
}
