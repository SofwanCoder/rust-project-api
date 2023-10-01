use crate::{database, emails};
use derive_more::DebugCustom;

#[derive(Clone, DebugCustom)]
#[debug(fmt = "ApplicationContext")]
pub struct ApplicationContext {
    pub db: database::ApplicationDatabase,
    pub email: emails::transports::Transports,
}
