use crate::models::Env;

pub mod api;
pub mod community;
pub mod errors;
pub mod htmx;
pub mod rate_limiter;
pub mod users;
pub mod validate;

impl From<String> for Env {
    fn from(env: String) -> Self {
        match env.as_str() {
            "development" => Self::Development,
            "testing" => Self::Testing,
            "production" => Self::Production,
            _ => panic!("Invalid environment"),
        }
    }
}
