use sea_orm::ConnectionTrait;

pub(crate) mod ampq;
pub(crate) mod mongo;
pub(crate) mod mysql;
pub(crate) mod postgres;
pub(crate) mod redis;

pub(crate) trait DBConnection = ConnectionTrait;

#[derive(Clone)]
pub struct ApplicationDatabase {
    pub postgres: postgres::ApplicationPostgresDatabase,
    pub redis: redis::ApplicationRedisDatabase,
    pub mongo: mongo::ApplicationMongoDatabase,
    pub ampq: ampq::ApplicationAmpqDatabase,
    pub mysql: mysql::ApplicationMysqlDatabase,
}

impl std::fmt::Debug for ApplicationDatabase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ApplicationDatabase").finish()
    }
}

impl ApplicationDatabase {
    pub(crate) async fn init() -> Self {
        Self {
            postgres: postgres::ApplicationPostgresDatabase::init().await,
            redis: redis::ApplicationRedisDatabase::init().await,
            mongo: mongo::ApplicationMongoDatabase::init().await,
            ampq: ampq::ApplicationAmpqDatabase::init().await,
            mysql: mysql::ApplicationMysqlDatabase::init().await,
        }
    }
}
