#![allow(dead_code)]

use crate::helpers::error::AppError;
use log::debug;

pub type PooledDatabaseConnection = sea_orm::DatabaseConnection;

#[derive(Debug, Clone)]
pub struct ApplicationMysqlDatabase {
    connection_pool: PooledDatabaseConnection,
}

impl ApplicationMysqlDatabase {
    pub(super) async fn init() -> Self {
        debug!("Initializing Mysql database with default settings");

        let database_url = crate::configs::settings::Variables::postgres_uri();

        let connection_pool = sea_orm::Database::connect(database_url).await;

        if connection_pool.is_err() {
            panic!("Failed to connect to Mysql database");
        }

        let connection_pool = connection_pool.unwrap();

        debug!("Mysql connection pool established");

        Self { connection_pool }
    }

    pub(crate) fn get_connection(&self) -> Result<PooledDatabaseConnection, AppError> {
        debug!("Getting Mysql connection from pool");
        return Ok(self.connection_pool.clone());
    }
}
