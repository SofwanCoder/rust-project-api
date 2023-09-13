#![allow(dead_code)]
mod manager;

use crate::helpers::error::AppError;
use log::debug;
use manager::AmpqConnectionManager;
use mobc::Pool;

const CACHE_POOL_MAX_OPEN: u64 = 16;
const CACHE_POOL_MAX_IDLE: u64 = 8;

pub type AmpqConnection = mobc::Connection<AmpqConnectionManager>;
pub type AmpqPool = Pool<AmpqConnectionManager>;

#[derive(Clone)]
pub struct ApplicationAmpqDatabase {
    connection_pool: AmpqPool,
}

impl ApplicationAmpqDatabase {
    pub async fn get_connection(&self) -> Result<AmpqConnection, AppError> {
        debug!("Getting ampq connection from pool");
        return self
            .connection_pool
            .get()
            .await
            .map_err(|e| AppError::database_error(e));
    }
}

impl Default for ApplicationAmpqDatabase {
    fn default() -> Self {
        debug!("Initializing ampq connection with default settings");
        let database_url = crate::configs::settings::Variables::ampq_uri();

        let manager = AmpqConnectionManager::new(database_url);

        let connection_pool = Pool::builder()
            .max_open(CACHE_POOL_MAX_OPEN)
            .max_idle(CACHE_POOL_MAX_IDLE)
            .build(manager);

        debug!("AMPQ connection pool established");

        ApplicationAmpqDatabase { connection_pool }
    }
}
