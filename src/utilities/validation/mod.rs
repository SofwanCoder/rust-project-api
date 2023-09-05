pub mod email;

use std::borrow::Cow;
use std::collections::HashMap;

pub fn gen_validation_error(message: &str) -> validator::ValidationError {
    validator::ValidationError {
        code: Cow::from("validation_error"),
        message: Some(Cow::from(message.to_string())),
        params: HashMap::new(),
    }
}
