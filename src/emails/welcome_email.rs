use crate::emails::Email;
use common::error::AppError;
use handlebars::Handlebars;
use lettre::{message::header::ContentType, AsyncTransport, Message};
use std::fmt::Debug;
use tracing::{debug, error, instrument};

#[derive(Debug)]
pub struct WelcomeEmail {
    pub to: String,
    pub name: String,
    pub code: String,
}
#[async_trait::async_trait]
impl Email for WelcomeEmail {
    #[instrument]
    async fn build(&self) -> Result<String, AppError> {
        debug!("Building WelcomeEmail");
        let mut handlebars = Handlebars::new();
        let templates = [
            ("templates.welcome-email", "./templates/welcome-email.hbs"),
            ("partials.styles", "./templates/partials/styles.hbs"),
            ("layouts.base", "./templates/layouts/base.hbs"),
        ];

        for (name, path) in templates.iter() {
            handlebars
                .register_template_file(name, path)
                .map_err(|e| AppError::internal_server(e))?;
        }

        let data = serde_json::json!({
            "name": &self.name,
            "code": self.code,
        });

        let body = handlebars
            .render("templates.welcome-email", &data)
            .map_err(|e| AppError::internal_server(e))?;

        Ok(body)
    }

    #[instrument]
    async fn send(
        &self,
        mailer: impl AsyncTransport + Send + Sync + Debug,
    ) -> Result<(), AppError> {
        debug!("Sending WelcomeEmail");

        let body = self.build().await?;
        let from = "Sofwan <hello@sofwan.com>"
            .parse()
            .map_err(|e| AppError::internal_server(e))?;
        let to = self.to.parse().map_err(|e| AppError::internal_server(e))?;

        let email = Message::builder()
            .from(from)
            .to(to)
            .subject("Welcome to project")
            .header(ContentType::TEXT_HTML)
            .body(body)
            .map_err(|e| AppError::internal_server(e))?;

        match mailer.send(email).await {
            Ok(_) => debug!("Email sent"),
            Err(_) => error!("Could not send email"),
        };

        Ok(())
    }
}
