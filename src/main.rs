use actix_web::{App, HttpServer};
mod configs;
mod contracts;
mod controllers;
mod helpers;
mod router;
mod services;
mod state;
mod types;
mod utilities;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(state::app::config())
            .app_data(configs::json::get_json_config())
            .service(router::get_router_scope())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
