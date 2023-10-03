use common::error::AppError;
use lettre::AsyncTransport;
use std::fmt::Debug;

pub(crate) mod password_changed_email;
pub(crate) mod welcome_email;

#[async_trait::async_trait]
pub(crate) trait Email {
    async fn build(&self) -> Result<String, AppError>;
    async fn send(&self, mailer: impl AsyncTransport + Send + Sync + Debug)
        -> Result<(), AppError>;
}
