use crate::repositories::user::UserRepository;
use crate::types::auths::AuthToken;
use crate::types::user::UserWithAuthInfo;
use crate::utilities::validation::ValidateWithDatabase;
use serde::{Deserialize, Serialize};
use validator::ValidateArgs;
use validator::{Validate, ValidationErrors};

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct CreateUserPayload {
    #[validate(length(min = 3, message = "Name must be greater than 3 chars"))]
    pub name: String,
    #[validate(
        email(message = "Email format is invalid"),
        custom(
            function = "validate_email",
            arg = "&'v_a crate::database::ApplicationDatabase",
        )
    )]
    pub email: String,
    #[validate(length(min = 6, message = "Password must be greater than 6 chars"))]
    pub password: String,
}
pub fn validate_email(
    email: &str,
    db: &crate::database::ApplicationDatabase,
) -> Result<(), validator::ValidationError> {
    let user = UserRepository::new(db.get_connection()).find_by_email(email.to_string());

    if user.is_err() {
        log::debug!(
            "Likely database error when validating email for user: {}",
            email
        );
        return Err(validator::ValidationError::new(
            "*Error occurred while validating email",
        ));
    }

    if user.unwrap().is_some() {
        return Err(validator::ValidationError::new("*Email already exists"));
    }

    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateUserResponse {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub authentication: AuthToken,
}

impl From<&UserWithAuthInfo> for CreateUserResponse {
    fn from(u: &UserWithAuthInfo) -> Self {
        CreateUserResponse {
            id: u.user.id,
            name: u.user.name.clone(),
            email: u.user.email.clone(),
            authentication: u.authentication.clone(),
        }
    }
}

impl ValidateWithDatabase for CreateUserPayload {
    fn validate_with_database(
        &self,
        args: &crate::database::ApplicationDatabase,
    ) -> Result<(), ValidationErrors> {
        return self.validate_args(&args);
    }
}
