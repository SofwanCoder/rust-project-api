use crate::pg::PooledDatabaseConnection;

pub mod mongo;
pub mod pg;
pub mod redis;

pub type DBConnection<'a> = &'a mut PooledDatabaseConnection;

#[derive(Clone, Default)]
pub struct ApplicationDatabase {
    pub pg: pg::ApplicationPgDatabase,
    pub redis: redis::ApplicationRedisDatabase,
    pub mongo: mongo::ApplicationMongoDatabase,
}
