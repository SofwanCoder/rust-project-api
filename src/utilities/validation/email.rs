use crate::repositories::user::UserRepository;
use crate::utilities::validation::gen_validation_error;

pub fn unique_email_validator(
    email: &str,
    db: &crate::database::ApplicationDatabase,
) -> Result<(), validator::ValidationError> {
    let connection = &mut db.get_connection();
    let (user, _) = UserRepository::find_by_email(connection, email.to_string());

    if user.is_some() {
        return Err(gen_validation_error("Email already exists"));
    }

    Ok(())
}
