use crate::helpers::error::AppError;
use actix_web::error::BlockingError;
use std::borrow::Cow;
use std::collections::HashMap;
use validator::{ValidationErrors, ValidationErrorsKind};

pub fn map_blocking_err_to_app_err(err: BlockingError) -> AppError {
    log::error!("Error: {:?}", err);
    AppError::internal_server("Error handling request".to_string())
}

pub fn map_validation_err_to_app_err(errors: ValidationErrors) -> AppError {
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

    let formatted_errors = format_input_errors(errors.errors());
    AppError::validation_error(String::from("Validation error"), Some(formatted_errors))
}
