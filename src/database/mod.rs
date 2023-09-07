use crate::pg::PooledDatabaseConnection;

pub mod pg;

pub type DBConnection<'a> = &'a mut PooledDatabaseConnection;
