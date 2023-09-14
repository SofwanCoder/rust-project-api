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

        errors.iter().for_each(|(field, error)| {
            if let ValidationErrorsKind::Field(errors) = error {
                if errors.len() == 0 {
                    return;
                }

                let message = errors[0]
                    .message
                    .clone()
                    .unwrap_or(Cow::from(format!("{} is invalid", field)))
                    .to_string();

                fields.insert(field, message);
            }
        });

        return fields;
    }

    let formatted_errors = format_input_errors(errors.errors());
    AppError::validation_error("validation error", Some(formatted_errors))
}

pub fn map_diesel_err_to_app_err(err: diesel::result::Error) -> AppError {
    log::error!("Error: {:?}", err);
    AppError::database_error("Error handling data")
}
