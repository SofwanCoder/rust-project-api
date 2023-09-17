use crate::emails::Email;
use handlebars::Handlebars;
use lettre::{message::header::ContentType, AsyncTransport, Message};
use std::fmt::Debug;
use tracing::{debug, error, instrument};

#[derive(Debug)]
pub struct WelcomeEmail {
    pub to: String,
    pub name: String,
}
#[async_trait::async_trait]
impl Email for WelcomeEmail {
    #[instrument]
    async fn build(&self) -> Result<String, Box<dyn std::error::Error>> {
        debug!("Building WelcomeEmail");
        let mut handlebars = Handlebars::new();
        let templates = [
            ("templates.welcome-email", "./templates/welcome-email.hbs"),
            ("partials.styles", "./templates/partials/styles.hbs"),
            ("layouts.base", "./templates/layouts/base.hbs"),
        ];

        for (name, path) in templates.iter() {
            handlebars.register_template_file(name, path)?;
        }

        let data = serde_json::json!({
            "name": &self.name,
            "code": "2367"
        });

        let body = handlebars.render("templates.welcome-email", &data)?;

        Ok(body)
    }

    #[instrument]
    async fn send(
        &self,
        mailer: impl AsyncTransport + Send + Sync + Debug,
    ) -> Result<(), Box<dyn std::error::Error>> {
        debug!("Sending WelcomeEmail");
        let body = self.build().await?;
        let email = Message::builder()
            .from("Sofwan <hello@sofwan.com>".parse()?)
            .to(self.to.parse()?)
            .to(self.to.parse()?)
            .subject("Welcome to project")
            .header(ContentType::TEXT_HTML)
            .body(body)?;

        match mailer.send(email).await {
            Ok(_) => debug!("Email sent"),
            Err(_) => error!("Could not send email"),
        };

        Ok(())
    }
}
