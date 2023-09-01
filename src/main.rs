use actix_web::http::StatusCode;
use actix_web::middleware::{ErrorHandlers, Logger};
use actix_web::{App, HttpServer};
use dotenv;

mod configs;
mod contracts;
mod controllers;
mod helpers;
mod middlewares;
mod router;
mod services;
mod state;
mod types;
mod utilities;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .wrap(middlewares::auths::Authorization::default())
            .wrap(Logger::default())
            .wrap(
                ErrorHandlers::new()
                    .default_handler(configs::app::error_default_handler)
                    .handler(StatusCode::NOT_FOUND, configs::app::error_404_handler),
            )
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
