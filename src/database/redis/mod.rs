#![allow(dead_code)]
mod manager;

use crate::{
    configs::constant::{CONNECTION_POOL_MAX_IDLE, CONNECTION_POOL_MAX_OPEN},
    helpers::error_helper::AppError,
};
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

impl std::fmt::Debug for ApplicationRedisDatabase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ApplicationRedisDatabase")
            .field("connection_pool", &"RedisPool")
            .finish()
    }
}

impl ApplicationRedisDatabase {
    pub(super) async fn init() -> Self {
        debug!("Initializing Redis database with default settings");
        let database_url = crate::configs::settings::Variables::redis_uri();

        let client = redis::Client::open(database_url).unwrap();

        let manager = RedisConnectionManager::new(client);

        let connection_pool = Pool::builder()
            .max_open(CONNECTION_POOL_MAX_OPEN as u64)
            .max_idle(CONNECTION_POOL_MAX_IDLE)
            .build(manager);

        debug!("Redis connection pool established");

        ApplicationRedisDatabase { connection_pool }
    }

    pub(crate) async fn get_connection(&self) -> Result<RedisConnection, AppError> {
        debug!("Getting redis connection");
        return self
            .connection_pool
            .get()
            .await
            .map_err(|e| AppError::connection_error(e));
    }
}
