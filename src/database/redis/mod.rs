#![allow(dead_code)]
mod manager;

use crate::configs::constant::{CONNECTION_POOL_MAX_IDLE, CONNECTION_POOL_MAX_OPEN};
use crate::helpers::error::AppError;
use log::debug;
use manager::RedisConnectionManager;
use mobc::Pool;
use redis;

pub type RedisConnection = mobc::Connection<RedisConnectionManager>;
pub type RedisPool = Pool<RedisConnectionManager>;

#[derive(Clone)]
pub struct ApplicationRedisDatabase {
    connection_pool: RedisPool,
}

impl ApplicationRedisDatabase {
    pub async fn get_connection(&self) -> Result<RedisConnection, AppError> {
        debug!("Getting redis connection");
        return self
            .connection_pool
            .get()
            .await
            .map_err(|e| AppError::database_error(e));
    }
}

impl Default for ApplicationRedisDatabase {
    fn default() -> Self {
        debug!("Initializing Redis database with default settings");
        let database_url = crate::configs::settings::Variables::redis_uri();

        let client = redis::Client::open(database_url).unwrap();

        let manager = RedisConnectionManager::new(client);

        let connection_pool = Pool::builder()
            .max_open(CONNECTION_POOL_MAX_OPEN as u64)
            .max_idle(CONNECTION_POOL_MAX_IDLE)
            .build(manager);

        ApplicationRedisDatabase { connection_pool }
    }
}
