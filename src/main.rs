use crate::database::mongo;
use actix_web::http::StatusCode;
use actix_web::middleware::{ErrorHandlers, Logger};
use actix_web::{App, HttpServer};
use database::{pg, redis};
use dotenv;

mod configs;
mod contracts;
mod controllers;
mod database;
mod helpers;
mod middlewares;
mod models;
mod repositories;
mod router;
mod schema;
mod services;
mod types;
mod utilities;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();
    let database = pg::ApplicationPgDatabase::default();
    let redis = redis::ApplicationRedisDatabase::default();
    let mongo = mongo::ApplicationMongoDatabase::default();
    HttpServer::new(move || {
        App::new()
            .wrap(middlewares::auths::Authorization::default())
            .wrap(Logger::default())
            .wrap(
                ErrorHandlers::new()
                    .default_handler(configs::app::error_default_handler)
                    .handler(StatusCode::NOT_FOUND, configs::app::error_404_handler),
            )
            .app_data(redis.clone())
            .app_data(database.clone())
            .app_data(mongo.clone())
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
