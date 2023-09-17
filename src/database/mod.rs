use derive_more::DebugCustom;
use sea_orm::ConnectionTrait;

pub(crate) mod ampq;
pub(crate) mod mongo;
pub(crate) mod mysql;
pub(crate) mod postgres;
pub(crate) mod redis;

pub(crate) trait DBConnection = ConnectionTrait;

#[derive(Clone, DebugCustom)]
#[debug(fmt = "ApplicationDatabase")]
pub struct ApplicationDatabase {
    pub postgres: postgres::ApplicationPostgresDatabase,
    pub redis: redis::ApplicationRedisDatabase,
    pub mongo: mongo::ApplicationMongoDatabase,
    pub ampq: ampq::ApplicationAmpqDatabase,
    pub mysql: mysql::ApplicationMysqlDatabase,
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
