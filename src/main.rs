#![feature(trait_alias)]

use dotenv;
use tracing::debug;
use tracing_log::LogTracer;

mod contracts;
mod controllers;
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

mod api;

pub type ApplicationContext = common::context::ApplicationContext;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    LogTracer::init().expect("Unable to setup logger");
    register_tracing_logger();

    debug!("Setting up application context");
    let app_context = ApplicationContext {
        db: common::database::ApplicationDatabase::init().await,
        email: Default::default(),
    };

    debug!("Setting up application events");
    events::start(app_context.clone())
        .await
        .expect("Unable to start event processing");

    debug!("Setting up application api");
    api::start(app_context).await
}

fn register_tracing_logger() {
    use tracing::subscriber::set_global_default;
    use tracing_subscriber::{fmt, prelude::*, EnvFilter, Registry};

    let layer = if common::configs::settings::Variables::environment().eq("production") {
        fmt::layer().json().boxed()
    } else {
        fmt::layer().boxed()
    };

    let subscriber = Registry::default()
        .with(EnvFilter::from_default_env())
        .with(layer);

    set_global_default(subscriber).unwrap();
}
