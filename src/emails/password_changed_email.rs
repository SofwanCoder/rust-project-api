use crate::emails::Email;
use handlebars::Handlebars;
use lettre::{message::header::ContentType, AsyncTransport, Message};
use std::fmt::Debug;
use tracing::{error, instrument};

#[derive(Debug)]
pub struct PasswordChangedEmail {
    pub to: String,
    pub name: String,
}
#[async_trait::async_trait]
impl Email for PasswordChangedEmail {
    #[instrument]
    async fn build(&self) -> Result<String, Box<dyn std::error::Error>> {
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
            handlebars.register_template_file(name, path)?;
        }

        let data = serde_json::json!({
            "name": &self.name,
        });

        let body = handlebars.render("templates.password-changed", &data)?;

        Ok(body)
    }

    async fn send(
        &self,
        mailer: impl AsyncTransport + Send + Sync + Debug,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let body = self.build().await?;
        let email = Message::builder()
            .from("Sofwan <hello@sofwan.com>".parse()?)
            .to(self.to.parse()?)
            .to(self.to.parse()?)
            .subject("Your password has been changed")
            .header(ContentType::TEXT_HTML)
            .body(body)?;

        match mailer.send(email).await {
            Ok(_) => {}
            Err(_) => error!("Could not send email"),
        };

        Ok(())
    }
}
