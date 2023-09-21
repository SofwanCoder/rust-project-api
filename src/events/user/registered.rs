use crate::{
    emails::{welcome_email::WelcomeEmail, Email},
    events::AppEvent,
    helpers::error::AppError,
    ApplicationContext,
};
use async_trait::async_trait;
use rand::Rng;
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
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
    #[instrument]
    async fn handle(&self, ctx: ApplicationContext) -> Result<(), AppError> {
        let random_code: u32 = rand::thread_rng().gen_range(100000..999999);

        let welcome_email = WelcomeEmail {
            to: self.email.clone(),
            name: self.name.clone(),
            code: random_code.to_string(),
        };

        welcome_email
            .send(ctx.email.smtp.sender.clone())
            .await
            .map(|_| info!("Welcome email sent to {}", self.email))
            .map_err(|e| AppError::internal_server(e))
    }
}
