use crate::helpers::error::AppError;
use lettre::AsyncTransport;
use std::fmt::Debug;

pub(crate) mod transports;

#[async_trait::async_trait]
pub(crate) trait Email {
    async fn build(&self) -> Result<String, AppError>;
    async fn send(&self, mailer: impl AsyncTransport + Send + Sync + Debug)
        -> Result<(), AppError>;
}
