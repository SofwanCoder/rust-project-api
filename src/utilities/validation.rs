use crate::helpers::error::AppError;
use actix_web::Result;
use std::borrow::Cow;
use std::collections::HashMap;
use validator::{Validate, ValidateArgs, ValidationErrors, ValidationErrorsKind};

pub trait ValidateWithDatabase {
    fn validate_with_database(
        &self,
        args: &crate::database::ApplicationDatabase,
    ) -> Result<(), validator::ValidationErrors>;
}

fn get_validated_err(errors: ValidationErrors) -> AppError {
    let formatted_errors = format_input_errors(errors.errors());
    AppError::validation_error(String::from("Validation error"), Some(formatted_errors))
}

pub fn validate_request_data<'v_a, T>(data: T) -> Result<(), AppError>
where
    T: Validate,
{
    let validated = data.validate();
    if validated.is_err() {
        return Err(get_validated_err(validated.err().unwrap()));
    }
    Ok(())
}

pub fn validate_request_with_database<'v_a, T>(
    data: T,
    db: &crate::database::ApplicationDatabase,
) -> Result<(), AppError>
where
    T: ValidateArgs<'v_a>,
    T: ValidateWithDatabase,
{
    let validated = data.validate_with_database(db);
    if validated.is_err() {
        return Err(get_validated_err(validated.err().unwrap()));
    }
    Ok(())
}

fn format_input_errors(
    errors: &HashMap<&'static str, ValidationErrorsKind>,
) -> HashMap<&'static str, String> {
    let mut fields: HashMap<&str, String> = HashMap::new();
    for (field, error) in errors.iter() {
        if let ValidationErrorsKind::Field(errors) = error {
            if errors.len() == 0 {
                continue;
            }
            let message = errors[0]
                .message
                .clone()
                .unwrap_or({
                    if errors[0].code.starts_with("*") {
                        Cow::from(&errors[0].code[1..])
                    } else {
                        Cow::from(format!("{} is invalid", field))
                    }
                })
                .to_string();

            fields.insert(field, message);
        }
    }
    return fields;
}
