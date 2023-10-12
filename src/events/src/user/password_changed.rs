use crate::{
    emails::{password_changed_email::PasswordChangedEmail, Email},
    AppEvent,
    ApplicationContext,
};
use async_trait::async_trait;
use common::error::AppError;
use derive_more::{Constructor, From};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, From, Constructor)]
pub struct UserPasswordChanged {
    pub user_id: Uuid,
    pub name: String,
    pub email: String,
}

#[async_trait]
impl AppEvent for UserPasswordChanged {
    #[instrument]
    async fn handle(&self, ctx: ApplicationContext) -> Result<(), AppError> {
        let password_changed_email = PasswordChangedEmail {
            to: self.email.clone(),
            name: self.name.clone(),
        };

        password_changed_email
            .send(ctx.email.smtp.sender.clone())
            .await
            .map(|_| info!("Password changed email sent to {}", self.email))
            .map_err(|e| AppError::internal_server(e))
    }
}
