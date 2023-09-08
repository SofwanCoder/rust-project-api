#![allow(dead_code)]
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::{r2d2, PgConnection};

type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type PooledDatabaseConnection = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Debug, Clone)]
pub struct ApplicationPgDatabase {
    connection_pool: DBPool,
}

impl ApplicationPgDatabase {
    pub fn get_connection(&self) -> PooledDatabaseConnection {
        return self.connection_pool.get().unwrap();
    }
}

impl Default for ApplicationPgDatabase {
    fn default() -> Self {
        let database_url = crate::configs::settings::Variables::postgres_url();
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let connection_pool: DBPool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        ApplicationPgDatabase { connection_pool }
    }
}
