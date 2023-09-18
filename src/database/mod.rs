use derive_more::DebugCustom;
use sea_orm::ConnectionTrait;

pub(crate) mod ampq;
pub(crate) mod mongo;
pub(crate) mod redis;
pub(crate) mod source;

pub(crate) trait DBConnection = ConnectionTrait;

#[derive(Clone, DebugCustom)]
#[debug(fmt = "ApplicationDatabase")]
pub struct ApplicationDatabase {
    pub source: source::ApplicationSourceDatabase,
    pub redis: redis::ApplicationRedisDatabase,
    pub mongo: mongo::ApplicationMongoDatabase,
    pub ampq: ampq::ApplicationAmpqDatabase,
}

impl ApplicationDatabase {
    pub(crate) async fn init() -> Self {
        Self {
            source: source::ApplicationSourceDatabase::init().await,
            redis: redis::ApplicationRedisDatabase::init().await,
            mongo: mongo::ApplicationMongoDatabase::init().await,
            ampq: ampq::ApplicationAmpqDatabase::init().await,
        }
    }
}
