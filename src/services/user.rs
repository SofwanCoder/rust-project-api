use crate::contracts::user::CreateUser;
use crate::helpers::error::AppError;

pub async fn register(body: CreateUser) -> Result<CreateUser, AppError> {
    Ok(body)
    // Err(AppError::new(
    //     "Invalid Body".to_string(),
    //     AppErrorKind::UserError,
    // ))
}
