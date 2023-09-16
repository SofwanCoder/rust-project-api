#![allow(dead_code)]
use crate::configs;
use crate::configs::constant::{CONNECTION_POOL_MAX_IDLE, CONNECTION_POOL_MAX_OPEN};
use crate::helpers::error::AppError;
use futures;
use log::debug;
use mongodb::{options::ClientOptions, Client, Database};

#[derive(Debug, Clone)]
pub struct ApplicationMongoDatabase {
    pub _db: Database,
}

impl ApplicationMongoDatabase {
    pub(super) async fn init() -> Self {
        debug!("Initializing Mongo database with default settings");

        let database_url = configs::settings::Variables::mongo_uri();

        let mut manager = futures::executor::block_on(ClientOptions::parse(database_url)).unwrap();
        manager.max_pool_size = Some(CONNECTION_POOL_MAX_OPEN as u32);
        manager.min_pool_size = Some(CONNECTION_POOL_MAX_IDLE as u32);

        let client: Client = Client::with_options(manager).unwrap();

        let _db = client
            .default_database()
            .unwrap_or(client.database("project_db"));

        debug!("Mongo connection pool established");

        ApplicationMongoDatabase { _db }
    }

    pub(crate) async fn get_connection(&self) -> Result<Database, AppError> {
        debug!("Getting Mongo connection from pool");
        return Ok(self._db.clone());
    }
}
