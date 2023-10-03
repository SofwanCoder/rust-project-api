#![allow(dead_code)]
mod manager;

use crate::{
    configs::constant::{CONNECTION_POOL_MAX_IDLE, CONNECTION_POOL_MAX_OPEN},
    error::AppError,
};
use derive_more::DebugCustom;
use manager::RedisConnectionManager;
use mobc::Pool;
use redis;
use tracing::debug;

pub type RedisConnection = mobc::Connection<RedisConnectionManager>;
pub type RedisPool = Pool<RedisConnectionManager>;

#[derive(Clone, DebugCustom)]
#[debug(fmt = "ApplicationRedisDatabase")]
pub struct ApplicationRedisDatabase {
    connection_pool: RedisPool,
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

    pub async fn get_connection(&self) -> Result<RedisConnection, AppError> {
        debug!("Getting redis connection");
        return self
            .connection_pool
            .get()
            .await
            .map_err(|e| AppError::connection_error(e));
    }
}
