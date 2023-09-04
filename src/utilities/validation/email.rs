use crate::repositories::user::UserRepository;
use crate::utilities::validation::gen_validation_error;

pub fn unique_email_validator(
    email: &str,
    db: &crate::database::ApplicationDatabase,
) -> Result<(), validator::ValidationError> {
    let user = UserRepository::new(db.get_connection()).find_by_email(email.to_string());

    if user.is_err() {
        log::debug!(
            "Likely database error when validating email for user: {}",
            email
        );
        return Err(gen_validation_error(
            "Error occurred while validating email",
        ));
    }

    if user.unwrap().is_some() {
        return Err(gen_validation_error("Email already exists"));
    }

    Ok(())
}
