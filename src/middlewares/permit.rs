#![allow(dead_code)]
use crate::{
    helpers::{response_helper, response_helper::AppResponse},
    types::auth_types::AuthenticatedData,
};
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::StatusCode,
    Error,
    HttpMessage,
};
use futures_util::future::{Either, LocalBoxFuture};
use log::debug;
use std::future::{ready, Ready};
use tracing::{instrument, trace};

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
    type Error = S::Error;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;
    type InitError = ();
    type Response = S::Response;
    type Transform = PermissionMiddleware<S>;

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
    type Error = S::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;
    type Response = S::Response;

    forward_ready!(service);

    #[instrument(fields(middlware = "PermissionMiddleware::call"), skip_all)]
    fn call(&self, req: ServiceRequest) -> Self::Future {
        let authenticated_user = req
            .extensions()
            .get::<AuthenticatedData>()
            .unwrap_or(&AuthenticatedData::default())
            .clone();

        if !authenticated_user.is_authenticated() {
            trace!("Authorization header is not present");
            return Box::pin(async move {
                let unauthorized_response = response_helper::app_http_response(
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

        // log the authenticated user
        debug!("Authenticated user: {:?}", authenticated_user);

        // check if the user_id in the url path is the same as the authenticated user
        let is_cross_access_request = {
            trace!("Checking if the user_id in the url path is the same as the authenticated user");
            let request_user_id = req.match_info().get("user_id");
            match request_user_id {
                None => false,
                Some(requested_user_id) => {
                    let authenticated_user_id = authenticated_user.user_id.to_string();
                    requested_user_id != authenticated_user_id
                }
            }
        };

        let is_permitted = !is_cross_access_request && authenticated_user.is_cleared(self.level);

        let either = if is_permitted {
            trace!("User is permitted to access the resource");
            Either::Right(self.service.call(req))
        } else {
            trace!("User is not permitted to access the resource");
            let forbidden_response = response_helper::app_http_response(
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
