use crate::configs;
use futures;
use log::debug;
use mongodb::{options::ClientOptions, Client, Database};

#[derive(Debug, Clone)]
pub struct ApplicationMongoDatabase {
    pub _db: Database,
}

impl Default for ApplicationMongoDatabase {
    fn default() -> Self {
        debug!("Initializing mongo database with default settings");
        let database_url = configs::settings::Variables::mongo_uri();
        let manager = futures::executor::block_on(ClientOptions::parse(database_url)).unwrap();
        let client: Client = Client::with_options(manager).unwrap();
        let _db = client
            .default_database()
            .unwrap_or(client.database("project_db"));
        ApplicationMongoDatabase { _db }
    }
}
