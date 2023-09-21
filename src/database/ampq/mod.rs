#![allow(dead_code)]
mod manager;

use crate::{
    configs_::constant::{CONNECTION_POOL_MAX_IDLE, CONNECTION_POOL_MAX_OPEN},
    helpers::error::AppError,
};
use derive_more::DebugCustom;
use log::debug;
use manager::AmpqConnectionManager;
use mobc::Pool;

pub type AmpqConnection = mobc::Connection<AmpqConnectionManager>;
pub type AmpqPool = Pool<AmpqConnectionManager>;

#[derive(Clone, DebugCustom)]
#[debug(fmt = "ApplicationAmpqDatabase")]
pub struct ApplicationAmpqDatabase {
    connection_pool: AmpqPool,
}

impl ApplicationAmpqDatabase {
    pub(super) async fn init() -> Self {
        debug!("Initializing AMPQ connection with default settings");

        let database_url = crate::configs_::settings::Variables::ampq_uri();

        let manager = AmpqConnectionManager::new(database_url);

        let connection_pool = Pool::builder()
            .max_open(CONNECTION_POOL_MAX_OPEN)
            .max_idle(CONNECTION_POOL_MAX_IDLE)
            .build(manager);

        debug!("AMPQ connection pool established");

        ApplicationAmpqDatabase { connection_pool }
    }

    pub(crate) async fn get_connection(&self) -> Result<AmpqConnection, AppError> {
        debug!("Getting AMPQ connection from pool");
        return self
            .connection_pool
            .get()
            .await
            .map_err(|e| AppError::connection_error(e));
    }
}
