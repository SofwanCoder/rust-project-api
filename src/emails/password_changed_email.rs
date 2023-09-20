use crate::{emails::Email, helpers::error::AppError};
use handlebars::Handlebars;
use lettre::{message::header::ContentType, AsyncTransport, Message};
use std::fmt::Debug;
use tracing::{debug, error, instrument};

#[derive(Debug)]
pub struct PasswordChangedEmail {
    pub to: String,
    pub name: String,
}
#[async_trait::async_trait]
impl Email for PasswordChangedEmail {
    #[instrument]
    async fn build(&self) -> Result<String, AppError> {
        debug!("Building PasswordChangedEmail");
        let mut handlebars = Handlebars::new();
        let templates = [
            (
                "templates.password-changed",
                "./templates/password-changed.hbs",
            ),
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
        });

        let body = handlebars
            .render("templates.password-changed", &data)
            .map_err(|e| AppError::internal_server(e))?;

        debug!("PasswordChangedEmail built");
        Ok(body)
    }

    #[instrument(skip(mailer))]
    async fn send(
        &self,
        mailer: impl AsyncTransport + Send + Sync + Debug,
    ) -> Result<(), AppError> {
        let body = self.build().await?;
        let from = "Sofwan <hello@sofwan.com>"
            .parse()
            .map_err(|e| AppError::internal_server(e))?;
        let to = self.to.parse().map_err(|e| AppError::internal_server(e))?;

        let email = Message::builder()
            .from(from)
            .to(to)
            .subject("Your password has been changed")
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
