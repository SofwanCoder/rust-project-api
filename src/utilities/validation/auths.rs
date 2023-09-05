pub fn validate_grant_type(grant_type: &str) -> Result<(), validator::ValidationError> {
    match grant_type {
        "password" => Ok(()),
        "refresh_token" => Ok(()),
        _ => Err(super::gen_validation_error("Grant type is invalid")),
    }
}
