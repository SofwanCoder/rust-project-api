use crate::{helpers, helpers::error_helper::AppError};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

pub fn hash(password: String) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default().hash_password(password.as_bytes(), &salt);

    if hash.is_err() {
        return Err(AppError::new(
            "Internal error when encrypting password",
            helpers::error_helper::AppErrorKind::InternalError,
        ));
    }

    return Ok(hash.unwrap().to_string());
}

pub fn verify(hash: String, password: String) -> Result<(), AppError> {
    let parsed_hash = PasswordHash::new(hash.as_str());
    if parsed_hash.is_err() {
        log::debug!(
            "InternalError::Existing password is an invalid hash: {:?}",
            parsed_hash.err()
        );
        return Err(AppError::internal_server("Existing password is invalid!"));
    }

    let result = Argon2::default().verify_password(password.as_bytes(), &parsed_hash.unwrap());
    if result.is_err() {
        return Err(AppError::client_error("Password does not match"));
    }

    return Ok(());
}
