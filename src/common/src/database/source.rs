#![allow(dead_code)]

use crate::error::AppError;
use sea_orm::ConnectOptions;
use tracing::{debug, log};

pub type PooledDatabaseConnection = sea_orm::DatabaseConnection;

#[derive(Debug, Clone)]
pub struct ApplicationSourceDatabase {
    connection_pool: PooledDatabaseConnection,
}

impl ApplicationSourceDatabase {
    pub(super) async fn init() -> Self {
        debug!("Initializing Postgres database with default settings");

        let database_url = crate::configs::settings::Variables::database_uri();
        let mut connection_opt = ConnectOptions::new(database_url);
        connection_opt
            .min_connections(1)
            .sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Debug);

        let connection_pool = sea_orm::Database::connect(connection_opt).await;

        if connection_pool.is_err() {
            panic!("Failed to connect to Postgres database");
        }

        let connection_pool = connection_pool.unwrap();

        debug!("Postgres connection pool established");

        Self { connection_pool }
    }

    pub async fn get_connection(&self) -> Result<PooledDatabaseConnection, AppError> {
        debug!("Getting Mysql connection from pool");
        return Ok(self.connection_pool.clone());
    }
}
