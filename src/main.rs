#![feature(trait_alias)]
use crate::{events::AppEvents, utilities::rand::generate_ulid};
use actix_web::{
    http::StatusCode,
    middleware::{ErrorHandlers, Logger},
    App,
    HttpServer,
};
use derive_more::{DebugCustom, Display};
use dotenv;
use std::fmt::Debug;
use tracing::info;
use tracing_log::LogTracer;
use ulid::Ulid;

mod configs;
mod contracts;
mod controllers;
mod database;
mod emails;
mod events;
mod helpers;
mod middlewares;
mod models;
mod repositories;
mod router;
mod services;
mod types;
mod utilities;

#[derive(Debug, Clone, Display)]
pub struct RequestId {
    id: Ulid,
}
impl Default for RequestId {
    fn default() -> Self {
        Self {
            id: generate_ulid(),
        }
    }
}

#[derive(Clone, DebugCustom)]
#[debug(fmt = "ApplicationDatabase")]
pub struct ApplicationContext {
    pub(crate) db: database::ApplicationDatabase,
    pub(crate) email: emails::transports::Transports,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    LogTracer::init().expect("Unable to setup logger");
    register_tracing_logger();

    let app_context = ApplicationContext {
        db: database::ApplicationDatabase::init().await,
        email: Default::default(),
    };

    AppEvents::init(app_context.clone())
        .await
        .expect("Unable to initialize events");

    let host = configs::settings::Variables::host();
    let port = configs::settings::Variables::port();

    info!("Starting server at http://{}:{}", host, port);

    HttpServer::new(move || {
        App::new()
            .wrap(middlewares::auth_middleware::Authorization::default())
            .wrap(
                ErrorHandlers::new()
                    .default_handler(configs::app::error_default_handler)
                    .handler(StatusCode::NOT_FOUND, configs::app::error_404_handler),
            )
            .wrap(Logger::default())
            .wrap(middlewares::request_middleware::AppRequest::default())
            .app_data(app_context.clone())
            .app_data(configs::json::get_json_config())
            .service(router::get_router_scope())
    })
    .bind((host, port))?
    .run()
    .await
}

fn register_tracing_logger() {
    use tracing::subscriber::set_global_default;
    use tracing_subscriber::{fmt, prelude::*, EnvFilter, Registry};

    let layer = if configs::settings::Variables::environment().eq("production") {
        fmt::layer().json().boxed()
    } else {
        fmt::layer().boxed()
    };

    let subscriber = Registry::default()
        .with(EnvFilter::from_default_env())
        .with(layer);

    set_global_default(subscriber).unwrap();
}
