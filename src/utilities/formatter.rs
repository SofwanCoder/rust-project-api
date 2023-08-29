use std::collections::HashMap;
use validator::ValidationErrorsKind;

pub fn input_errors(
    errors: &HashMap<&'static str, ValidationErrorsKind>,
) -> HashMap<&'static str, String> {
    let mut fields: HashMap<&str, String> = HashMap::new();
    for (field, error) in errors.iter() {
        if let ValidationErrorsKind::Field(errors) = error {
            let mut error_message = String::new();
            for error in errors.iter() {
                let ers = error.message.clone().unwrap().to_string();
                error_message.push_str(ers.as_str());
            }
            fields.insert(field, error_message);
        }
    }
    return fields;
}
