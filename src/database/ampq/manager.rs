use lapin::{Connection, ConnectionProperties, ConnectionState, Error};
use log::debug;
use mobc::{async_trait, Manager};

#[derive(Debug, Clone)]
pub struct AmpqConnectionManager {
    client: String,
}

impl AmpqConnectionManager {
    pub fn new(client: String) -> Self {
        Self { client }
    }
}

#[async_trait]
impl Manager for AmpqConnectionManager {
    type Connection = Connection;
    type Error = Error;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        debug!("Establishing new AMPQ connection");
        let conn = Connection::connect(&self.client, ConnectionProperties::default()).await?;
        Ok(conn)
    }

    async fn check(&self, conn: Self::Connection) -> Result<Self::Connection, Self::Error> {
        debug!("Checking AMPQ connection status");
        let connection_status = conn.status();
        if connection_status.state() != ConnectionState::Connected {
            return Err(Error::InvalidConnectionState(ConnectionState::Closed));
        }
        Ok(conn)
    }
}
