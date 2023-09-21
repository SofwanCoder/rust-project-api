use super::error::{AppError, AppErrorKind};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use tracing::{debug, error, instrument};

#[instrument(skip_all)]
pub fn hash_password(password: String) -> Result<String, AppError> {
    debug!("Hashing password");
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default().hash_password(password.as_bytes(), &salt);

    if hash.is_err() {
        error!("Internal error when encrypting password: {:?}", hash.err());
        return Err(AppError::new(
            "Internal error when encrypting password",
            AppErrorKind::InternalError,
        ));
    }

    debug!("Password hashed");
    return Ok(hash.unwrap().to_string());
}

#[instrument(skip_all)]
pub fn verify_password(hash: String, password: String) -> Result<(), AppError> {
    debug!("Verifying password from hash");

    let parsed_hash = PasswordHash::new(hash.as_str());
    if parsed_hash.is_err() {
        error!(
            "InternalError::Existing password is an invalid hash: {:?}",
            parsed_hash.err()
        );
        return Err(AppError::internal_server("Existing password is invalid!"));
    }

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash.unwrap())
        .map_err(|err| {
            error!("InternalError::Error when verifying password: {:?}", err);
            AppError::client_error("Password does not match")
        })?;

    debug!("Password verified from hash");
    return Ok(());
}
