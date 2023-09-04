pub mod email;

use crate::helpers::error::AppError;
use std::borrow::Cow;
use std::collections::HashMap;
use validator::{ValidationErrors, ValidationErrorsKind};

pub fn map_to_validation_err(errors: ValidationErrors) -> AppError {
    let formatted_errors = format_input_errors(errors.errors());
    AppError::validation_error(String::from("Validation error"), Some(formatted_errors))
}

pub fn gen_validation_error(message: &str) -> validator::ValidationError {
    validator::ValidationError {
        code: Cow::from("validation_error"),
        message: Some(Cow::from(message.to_string())),
        params: HashMap::new(),
    }
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
                .unwrap_or(Cow::from(format!("{} is invalid", field)))
                .to_string();

            fields.insert(field, message);
        }
    }
    return fields;
}
