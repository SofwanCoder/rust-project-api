use crate::models::auth::AuthModel;
use chrono::Duration;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthenticatedData {
    pub session_id: Uuid,
    pub user_id: Uuid,
    pub clearance_level: u8,
    pub iat: usize,
    pub exp: usize,
}

impl AuthenticatedData {
    pub fn is_cleared(&self, level: u8) -> bool {
        self.clearance_level >= level
    }

    pub fn is_admin(&self) -> bool {
        self.clearance_level >= 5
    }

    pub fn is_authenticated(&self) -> bool {
        !self.user_id.is_nil()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RefreshTokenData {
    pub user_id: Uuid,
    pub token_id: Uuid,
    iat: usize,
    exp: usize,
}

impl From<&AuthModel> for RefreshTokenData {
    fn from(value: &AuthModel) -> Self {
        RefreshTokenData {
            user_id: value.user_id,
            token_id: value.id,
            iat: chrono::Utc::now().timestamp() as usize,
            exp: value.expires_at.timestamp() as usize,
        }
    }
}
