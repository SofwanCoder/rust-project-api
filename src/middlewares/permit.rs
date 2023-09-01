use crate::helpers::response;
use crate::helpers::response::AppResponse;
use crate::types::auths::AuthenticatedData;
use actix_web::http::StatusCode;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures_util::future::{Either, LocalBoxFuture};
use std::future::{ready, Ready};

pub struct Clearance;

pub struct Permission<T = Clearance> {
    pub level: u8,
    _marker: std::marker::PhantomData<T>,
}

impl Default for Permission {
    fn default() -> Self {
        Permission {
            level: 0,
            _marker: std::marker::PhantomData,
        }
    }
}

impl Permission<Clearance> {
    pub fn allow(level: u8) -> Self {
        Permission {
            level,
            ..Default::default()
        }
    }
}

impl<S> Transform<S, ServiceRequest> for Permission
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Transform = PermissionMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;
    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(PermissionMiddleware {
            service,
            level: self.level,
        }))
    }
}
pub struct PermissionMiddleware<S> {
    service: S,
    level: u8,
}
impl<S> Service<ServiceRequest> for PermissionMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let authenticated_user = req
            .extensions()
            .get::<AuthenticatedData>()
            .unwrap_or(&AuthenticatedData::blank())
            .clone();

        if !authenticated_user.is_authenticated() {
            return Box::pin(async move {
                let unauthorized_response = response::ret(
                    StatusCode::UNAUTHORIZED,
                    AppResponse::<()> {
                        message: "No Authorization found".to_string(),
                        data: None,
                        errors: None,
                    },
                );
                Ok(req.into_response(unauthorized_response))
            });
        }

        let either = if authenticated_user.is_cleared(self.level) {
            Either::Right(self.service.call(req))
        } else {
            let forbidden_response = response::ret(
                StatusCode::FORBIDDEN,
                AppResponse::<()> {
                    message: "Insufficient Access Permission".to_string(),
                    data: None,
                    errors: None,
                },
            );
            Either::Left(req.into_response(forbidden_response))
        };

        return Box::pin(async move {
            return match either {
                Either::Left(res) => Ok(res),
                Either::Right(fut) => fut.await,
            };
        });
    }
}
