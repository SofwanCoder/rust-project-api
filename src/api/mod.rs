use crate::{middlewares, router, ApplicationContext};
use actix_web::{
    http::StatusCode,
    middleware::{ErrorHandlers, Logger},
    App,
    Error,
    FromRequest,
    HttpMessage,
    HttpResponse,
    HttpServer,
    Responder,
    ResponseError,
};
use common::{configs, error::AppError, rand::generate_ulid};
use derive_more::Display;
use tracing::info;
use ulid::Ulid;

// R impl Responder
// E impl ResponseError
pub type ApiResult<R = HttpResponse, E = AppError> = Result<R, E>;

pub async fn start(app_context: ApplicationContext) -> std::io::Result<()> {
    let host = configs::settings::Variables::host();
    let port = configs::settings::Variables::port();

    info!("Starting server at http://{}:{}", host, port);

    HttpServer::new(move || {
        App::new()
            .wrap(middlewares::auth::Authorization::default())
            .wrap(
                ErrorHandlers::new()
                    .default_handler(configs::app::error_default_handler)
                    .handler(StatusCode::NOT_FOUND, configs::app::error_404_handler),
            )
            .wrap(Logger::default())
            .wrap(middlewares::request::AppRequest::default())
            .app_data(app_context.clone())
            .app_data(configs::json::get_json_config())
            .service(router::get_router_scope())
    })
    .bind((host, port))?
    .run()
    .await
}

#[derive(Debug, Clone, Display)]
#[display(fmt = "{}", _0)]
pub struct RequestId(pub Ulid);

impl Default for RequestId {
    fn default() -> Self {
        Self(generate_ulid())
    }
}

impl FromRequest for RequestId {
    type Error = Error;
    type Future = futures::future::Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let ext = req.extensions();
        let request_id = ext.get::<RequestId>().unwrap();
        futures::future::ok(request_id.clone())
    }
}
