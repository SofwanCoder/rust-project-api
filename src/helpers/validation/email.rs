use crate::{helpers::validation::gen_validation_error, repositories::user::UserRepository};
use tracing::instrument;

#[instrument(skip_all)]
pub fn unique_email_validator(
    email: &str,
    db: &common::database::ApplicationDatabase,
) -> Result<(), validator::ValidationError> {
    futures::executor::block_on(validate_unique_email(email, db))
}

#[instrument(skip_all)]
async fn validate_unique_email(
    email: &str,
    db: &common::database::ApplicationDatabase,
) -> Result<(), validator::ValidationError> {
    let connection = &db
        .source
        .get_connection()
        .await
        .map_err(|e| gen_validation_error(&e.to_string()))?;
    let user = UserRepository::find_by_email(connection, email.to_string()).await;

    if user.is_some() {
        return Err(gen_validation_error("Email already exists"));
    }

    Ok(())
}
