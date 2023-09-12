use lettre::AsyncTransport;

pub(crate) mod transports;
pub(crate) mod welcome_email;

#[async_trait::async_trait]
pub(crate) trait Email {
    async fn build(&self) -> Result<String, Box<dyn std::error::Error>>;
    async fn send(
        &self,
        mailer: impl AsyncTransport + Send + Sync,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
