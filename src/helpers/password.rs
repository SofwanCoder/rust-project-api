use crate::helpers::error::AppError;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

pub fn hash(password: &[u8]) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default().hash_password(password, &salt);

    if hash.is_err() {
        log::debug!("Error hashing password: {:?}", hash.err());
        return Err(AppError::new(
            "Internal error when encrypting password".to_string(),
            crate::helpers::error::AppErrorKind::InternalError,
        ));
    }

    return Ok(hash.unwrap().to_string());
}

pub fn verify(hash: &str, password: &[u8]) -> Result<(), AppError> {
    let parsed_hash = PasswordHash::new(hash);

    if parsed_hash.is_err() {
        log::debug!(
            "Existing password is an invalid hash: {:?}",
            parsed_hash.err()
        );
        return Err(AppError::new(
            "Existing password is invalid!".to_string(),
            crate::helpers::error::AppErrorKind::InternalError,
        ));
    }

    let result = Argon2::default().verify_password(password, &parsed_hash.unwrap());

    if result.is_err() {
        return Err(AppError::new(
            "Password does not match".to_string(),
            crate::helpers::error::AppErrorKind::ClientError,
        ));
    }

    return Ok(());
}
