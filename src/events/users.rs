use crate::emails::welcome_email::WelcomeEmail;
use crate::emails::Email;
use crate::events::AppEvent;
use crate::helpers::error::AppError;
use crate::ApplicationContext;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct UserRegistered {
    pub user_id: Uuid,
    pub name: String,
    pub email: String,
}

impl UserRegistered {
    // new
    pub fn new(user_id: Uuid, name: String, email: String) -> Self {
        Self {
            user_id,
            name,
            email,
        }
    }
}

#[async_trait]
impl AppEvent for UserRegistered {
    async fn handle(&self, ctx: ApplicationContext) -> Result<(), AppError> {
        let welcome_email = WelcomeEmail {
            to: self.email.clone(),
            name: self.name.clone(),
        };

        let _ = welcome_email
            .send(ctx.email.smtp.sender.clone())
            .await
            .map(|_| println!("Welcome email sent"))
            .map_err(|_| AppError::internal_server("Failed to send welcome email".to_string()));

        Ok(())
    }
}
