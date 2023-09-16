use smtp::SmtpAppTransport;
use std::fmt::Debug;

mod smtp;

#[derive(Clone, Default)]
pub struct Transports {
    pub smtp: SmtpAppTransport,
}

impl Debug for Transports {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Transports")
            .field("smtp", &"SmtpAppTransport")
            .finish()
    }
}
