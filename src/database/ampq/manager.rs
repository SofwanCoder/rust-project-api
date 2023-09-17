use lapin::{Connection, ConnectionProperties, ConnectionState, Error};
use mobc::{async_trait, Manager};
use tracing::debug;

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
        debug!("Checking AMPQ connection");
        let connection_status = conn.status();
        if connection_status.state() != ConnectionState::Connected {
            debug!(
                "AMPQ connection is not connected ({:?})",
                connection_status.state()
            );
            return Err(Error::InvalidConnectionState(ConnectionState::Closed));
        }
        debug!("AMPQ connection is ok");
        Ok(conn)
    }
}
