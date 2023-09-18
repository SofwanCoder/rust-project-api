use derive_more::DebugCustom;
use smtp::SmtpAppTransport;

mod smtp;

#[derive(Clone, Default, DebugCustom)]
#[debug(fmt = "EmailTransports")]
pub struct Transports {
    pub smtp: SmtpAppTransport,
}
