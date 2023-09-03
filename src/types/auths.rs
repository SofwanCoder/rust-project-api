use chrono::Duration;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticatedData {
    pub user_id: Uuid,
    pub clearance_level: u8,
    pub iat: usize,
    pub exp: usize,
}

impl Default for AuthenticatedData {
    fn default() -> Self {
        AuthenticatedData {
            user_id: uuid::Uuid::nil(),
            clearance_level: 0,
            // Issued now
            iat: chrono::Utc::now().timestamp() as usize,
            // Expires In 24hrs
            exp: (chrono::Utc::now().timestamp() + chrono::Duration::days(1).num_seconds())
                as usize,
        }
    }
}

impl AuthenticatedData {
    pub fn blank() -> Self {
        AuthenticatedData {
            user_id: uuid::Uuid::nil(),
            clearance_level: 0,
            iat: 0,
            exp: 0,
        }
    }

    pub fn is_cleared(&self, level: u8) -> bool {
        self.clearance_level >= level
    }

    pub fn is_authenticated(&self) -> bool {
        !self.user_id.is_nil()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthToken {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: usize,
    pub token_type: String,
}

impl AuthToken {
    pub fn new(access_token: String, refresh_token: String) -> Self {
        AuthToken {
            access_token,
            refresh_token,
            expires_in: Duration::days(1).num_seconds() as usize,
            token_type: "Bearer".to_string(),
        }
    }
}
