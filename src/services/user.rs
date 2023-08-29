use crate::contracts::user::CreateUser;
use crate::helpers::error::{AppError, AppErrorKind};
use std::any::Any;

pub async fn register(body: CreateUser) -> Result<CreateUser, AppError> {
    println!("user::create");

    Ok(body)
    // Err(AppError::new(
    //     "Invalid Body".to_string(),
    //     AppErrorKind::UserError,
    // ))
}
