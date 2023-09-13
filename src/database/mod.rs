use crate::pg::PooledDatabaseConnection;

pub(crate) mod ampq;
pub(crate) mod mongo;
pub(crate) mod pg;
pub(crate) mod redis;

pub(crate) type DBConnection<'a> = &'a mut PooledDatabaseConnection;

#[derive(Clone, Default)]
pub struct ApplicationDatabase {
    pub pg: pg::ApplicationPgDatabase,
    pub redis: redis::ApplicationRedisDatabase,
    pub mongo: mongo::ApplicationMongoDatabase,
    pub ampq: ampq::ApplicationAmpqDatabase,
}
