use derive_more::DebugCustom;
use smtp::SmtpAppTransport;
use std::fmt::Debug;

mod smtp;

#[derive(Clone, Default, DebugCustom)]
#[debug(fmt = "SmtpTransports")]
pub struct Transports {
    pub smtp: SmtpAppTransport,
}
