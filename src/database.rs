use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::{r2d2, PgConnection};

type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DBConnection = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct ApplicationDatabase {
    connection_pool: DBPool,
}

impl ApplicationDatabase {
    pub fn get_connection(&self) -> DBConnection {
        return self.connection_pool.get().unwrap();
    }
}

impl Default for ApplicationDatabase {
    fn default() -> Self {
        let database_url = crate::configs::settings::Variables::database_url();
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let connection_pool: DBPool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        ApplicationDatabase { connection_pool }
    }
}
