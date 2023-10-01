use derive_more::DebugCustom;
use sea_orm::ConnectionTrait;

pub mod ampq;
pub mod mongo;
pub mod redis;
pub mod source;

pub trait DBConnection = ConnectionTrait;

#[derive(Clone, DebugCustom)]
#[debug(fmt = "ApplicationDatabase")]
pub struct ApplicationDatabase {
    pub source: source::ApplicationSourceDatabase,
    pub redis: redis::ApplicationRedisDatabase,
    pub mongo: mongo::ApplicationMongoDatabase,
    pub ampq: ampq::ApplicationAmpqDatabase,
}

impl ApplicationDatabase {
    pub async fn init() -> Self {
        Self {
            source: source::ApplicationSourceDatabase::init().await,
            redis: redis::ApplicationRedisDatabase::init().await,
            mongo: mongo::ApplicationMongoDatabase::init().await,
            ampq: ampq::ApplicationAmpqDatabase::init().await,
        }
    }
}
