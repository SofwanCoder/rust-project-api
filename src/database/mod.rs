use crate::pg::PooledDatabaseConnection;

pub mod pg;
pub mod redis;

pub type DBConnection<'a> = &'a mut PooledDatabaseConnection;
