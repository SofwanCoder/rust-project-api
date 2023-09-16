use crate::helpers::validation::gen_validation_error;
use crate::repositories::user_repository::UserRepository;

pub fn unique_email_validator(
    email: &str,
    db: &crate::database::ApplicationDatabase,
) -> Result<(), validator::ValidationError> {
    futures::executor::block_on(unique_email(email, db))
}

async fn unique_email(
    email: &str,
    db: &crate::database::ApplicationDatabase,
) -> Result<(), validator::ValidationError> {
    let connection = &db
        .postgres
        .get_connection()
        .await
        .map_err(|e| gen_validation_error(&e.to_string()))?;
    let user = UserRepository::find_by_email(connection, email.to_string()).await;

    if user.is_some() {
        return Err(gen_validation_error("Email already exists"));
    }

    Ok(())
}
