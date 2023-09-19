use crate::contracts::auth_contract::{CreateTokenPayload, GrantType};

pub fn validate_grant_type(
    grant_type: &GrantType,
    payload: &CreateTokenPayload,
) -> Result<(), validator::ValidationError> {
    if ![GrantType::Password, GrantType::RefreshToken].contains(&grant_type) {
        return Err(super::gen_validation_error("Grant type is invalid"));
    }

    match grant_type {
        GrantType::Password => {
            if payload.email.is_none() || payload.password.is_none() {
                return Err(super::gen_validation_error(
                    "Grant type is invalid for this request",
                ));
            }
        }
        GrantType::RefreshToken => {
            if payload.refresh_token.is_none() {
                return Err(super::gen_validation_error(
                    "Grant type is invalid for this request",
                ));
            }
        }

        _ => {
            return Err(super::gen_validation_error(
                "Grant type is invalid for this request",
            ));
        }
    }

    Ok(())
}
