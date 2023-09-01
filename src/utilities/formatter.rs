use std::collections::HashMap;
use validator::ValidationErrorsKind;

pub fn input_errors(
    errors: &HashMap<&'static str, ValidationErrorsKind>,
) -> HashMap<&'static str, String> {
    let mut fields: HashMap<&str, String> = HashMap::new();
    for (field, error) in errors.iter() {
        if let ValidationErrorsKind::Field(errors) = error {
            fields.insert(field, errors[0].message.clone().unwrap().to_string());
        }
    }
    return fields;
}
