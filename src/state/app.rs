use crate::types::state::AppConfig;

pub fn config() -> AppConfig {
    AppConfig {
        name: String::from("app"),
    }
}
