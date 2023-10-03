use common::error::AppError;
use std::{borrow::Cow, collections::HashMap, error::Error};
use validator::{ValidationErrors, ValidationErrorsKind};

pub fn map_err_to_internal_err(err: impl Error) -> AppError {
    tracing::error!("Error: {:?}", err);
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

#[cfg(test)]
mod tests {
    use super::*;
    use validator::ValidationError;

    #[test]
    fn test_map_validation_err_to_app_err() {
        let mut validation_errors = ValidationErrors::new();
        validation_errors.add(
            "email",
            ValidationError {
                code: Cow::from("email"),
                message: Some(Cow::from("email is invalid")),
                params: HashMap::new(),
            },
        );

        let formatted_errors = map_validation_err_to_app_err(validation_errors);
        assert_eq!(formatted_errors.data.as_ref().unwrap().len(), 1);
        assert_eq!(formatted_errors.message, "validation error");
        assert_eq!(
            formatted_errors.data.unwrap().get("email").unwrap(),
            "email is invalid"
        );
    }

    #[test]
    fn test_map_err_to_internal_err() {
        let err = map_err_to_internal_err(AppError::internal_server(""));
        assert_eq!(err.kind, crate::error::AppErrorKind::InternalError);
        assert_eq!(err.message, "Error handling request");
    }
}
