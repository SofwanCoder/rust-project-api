use crate::helpers::error::AppError;
use crate::utilities::formatter;
use actix_web::Result;
use validator::Validate;
pub fn validate_request_data<T>(data: T) -> Result<(), AppError>
where
    T: Validate,
{
    let validated = data.validate();
    if validated.is_err() {
        let errors = validated.err().unwrap();
        let formatted_errors = formatter::input_errors(errors.errors());
        return Err(AppError::validation_error(
            String::from("Validation error"),
            Some(formatted_errors),
        ));
    }
    Ok(())
}
