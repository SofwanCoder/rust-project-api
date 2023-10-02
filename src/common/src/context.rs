use crate::{database, drivers};
use derive_more::DebugCustom;

#[derive(Clone, DebugCustom)]
#[debug(fmt = "ApplicationContext")]
pub struct ApplicationContext {
    pub db: database::ApplicationDatabase,
    pub email: drivers::mailer::Transports,
}
