use crate::contracts::auth_contract::CreateTokenPayload;

pub fn validate_grant_type(
    grant_type: &str,
    payload: &CreateTokenPayload,
) -> Result<(), validator::ValidationError> {
    if !["password", "refresh_token"].contains(&grant_type) {
        return Err(super::gen_validation_error("Grant type is invalid"));
    }

    // If grant type is password, email and password are required
    if grant_type.eq("password")
        && payload
            .email
            .clone()
            .and(payload.password.clone())
            .is_none()
    {
        return Err(super::gen_validation_error(
            "Grant type is invalid for this request",
        ));
    }

    // If grant type is refresh_token, email and password are required
    if grant_type.eq("refresh_token") && payload.refresh_token.clone().is_none() {
        return Err(super::gen_validation_error(
            "Grant type is invalid for this request",
        ));
    }

    Ok(())
}
