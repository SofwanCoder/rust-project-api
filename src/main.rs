use actix_web::http::StatusCode;
use actix_web::middleware::{ErrorHandlers, Logger};
use actix_web::{App, HttpServer};
use database::pg;
use dotenv;

mod configs;
mod contracts;
mod controllers;
mod database;
mod emails;
mod helpers;
mod middlewares;
mod models;
mod repositories;
mod router;
mod schema;
mod services;
mod types;
mod utilities;

#[derive(Clone, Default)]
pub struct ApplicationContext {
    pub(crate) db: database::ApplicationDatabase,
    pub(crate) email: emails::transports::Transports,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let app_context = ApplicationContext::default();

    HttpServer::new(move || {
        App::new()
            .wrap(middlewares::auths::Authorization::default())
            .wrap(Logger::default())
            .wrap(
                ErrorHandlers::new()
                    .default_handler(configs::app::error_default_handler)
                    .handler(StatusCode::NOT_FOUND, configs::app::error_404_handler),
            )
            .app_data(app_context.clone())
            .app_data(configs::json::get_json_config())
            .service(router::get_router_scope())
    })
    .bind((
        configs::settings::Variables::host(),
        configs::settings::Variables::port(),
    ))?
    .run()
    .await
}
