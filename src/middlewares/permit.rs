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

pub struct Authenticated; // Only the resource owner can access
pub struct Administrator; // Only the admin can access

#[derive(Debug, Clone)]
enum PermissionKind {
    Authenticated,
    Administrator,
}

pub struct Permission<T = Authenticated> {
    level: u8,
    kind: PermissionKind,
    _marker: std::marker::PhantomData<T>,
}

impl Permission {
    pub fn allow(level: u8) -> Self {
        Permission::<Authenticated> {
            level,
            kind: PermissionKind::Authenticated,
            ..Default::default()
        }
    }
}

impl Permission<Administrator> {
    pub fn allow(level: u8) -> Self {
        Permission::<Administrator> {
            level,
            kind: PermissionKind::Administrator,
            ..Default::default()
        }
    }
}

impl Default for Permission {
    fn default() -> Self {
        Permission {
            level: 1,
            kind: PermissionKind::Authenticated, // Only the resource owner can access
            _marker: std::marker::PhantomData,
        }
    }
}

impl Default for Permission<Administrator> {
    fn default() -> Self {
        Permission {
            level: 1,
            kind: PermissionKind::Administrator, // Only the admin can access
            _marker: std::marker::PhantomData,
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
            kind: self.kind.clone(),
            level: self.level,
        }))
    }
}
pub struct PermissionMiddleware<S> {
    service: S,
    level: u8,
    kind: PermissionKind,
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
            .unwrap_or(&AuthenticatedData::default())
            .clone();

        if !authenticated_user.is_authenticated() {
            return Box::pin(async move {
                let unauthorized_response = response::app_http_response(
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

        // check if the user_id in the url path is the same as the authenticated user
        let is_cross_access_request = {
            let request_user_id = req.match_info().get("user_id");
            match request_user_id {
                None => false,
                Some(requested_user_id) => {
                    let authenticated_user_id = authenticated_user.user_id.to_string();
                    requested_user_id != authenticated_user_id
                }
            }
        };

        let is_permitted = {
            if is_cross_access_request {
                match self.kind {
                    PermissionKind::Authenticated => false,
                    PermissionKind::Administrator => authenticated_user.is_admin(),
                }
            } else {
                authenticated_user.is_cleared(self.level)
            }
        };

        let either = if is_permitted {
            Either::Right(self.service.call(req))
        } else {
            let forbidden_response = response::app_http_response(
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
