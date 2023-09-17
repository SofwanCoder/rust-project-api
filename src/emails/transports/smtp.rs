use lettre::{
    transport::smtp::{
        authentication::{Credentials, Mechanism},
        PoolConfig,
    },
    AsyncSmtpTransport,
    Tokio1Executor,
};
use url::Url;

#[derive(Clone, Debug)]
pub struct SmtpAppTransport {
    pub sender: AsyncSmtpTransport<Tokio1Executor>,
}

impl Default for SmtpAppTransport {
    fn default() -> Self {
        let smtp_uri = crate::configs::settings::Variables::smtp_uri();
        let parsed_smtp_uri = Url::parse(&smtp_uri).unwrap();
        let smtp_host = parsed_smtp_uri.host_str().unwrap();
        let username = parsed_smtp_uri.username();
        let password = parsed_smtp_uri.password().unwrap();
        let sender = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(smtp_host)
            .unwrap_or(AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(
                smtp_host,
            ))
            .credentials(Credentials::new(username.to_string(), password.to_string()))
            .authentication(vec![Mechanism::Plain])
            .pool_config(PoolConfig::new().max_size(20))
            .build();

        Self { sender }
    }
}
