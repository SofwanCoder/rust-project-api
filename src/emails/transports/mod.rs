use smtp::SmtpAppTransport;

mod smtp;

#[derive(Clone, Default)]
pub struct Transports {
    pub smtp: SmtpAppTransport,
}
