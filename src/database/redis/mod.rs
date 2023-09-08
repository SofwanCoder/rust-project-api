#![allow(dead_code)]
mod manager;
use crate::helpers::error::AppError;
use manager::RedisConnectionManager;
use mobc::Pool;
use redis;

const CACHE_POOL_MAX_OPEN: u64 = 16;
const CACHE_POOL_MAX_IDLE: u64 = 8;

pub type RedisConnection = mobc::Connection<RedisConnectionManager>;
pub type RedisPool = Pool<RedisConnectionManager>;

#[derive(Clone)]
pub struct ApplicationRedisDatabase {
    connection_pool: RedisPool,
}

impl ApplicationRedisDatabase {
    pub async fn get_connection(&self) -> Result<RedisConnection, AppError> {
        return self
            .connection_pool
            .get()
            .await
            .map_err(|e| AppError::database_error(e));
    }
}

impl Default for ApplicationRedisDatabase {
    fn default() -> Self {
        let database_url = crate::configs::settings::Variables::redis_uri();

        let client = redis::Client::open(database_url).unwrap();

        let manager = RedisConnectionManager::new(client);

        let connection_pool = Pool::builder()
            .max_open(CACHE_POOL_MAX_OPEN)
            .max_idle(CACHE_POOL_MAX_IDLE)
            .build(manager);

        ApplicationRedisDatabase { connection_pool }
    }
}
