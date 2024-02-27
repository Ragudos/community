use crate::models::{
    RECAPTCHA_CLIENT_SITEKEY, RECAPTCHA_SECRET_KEY_FOR_TESTS, RECAPTCHA_SITEKEY_FOR_TESTS,
};

pub mod db;
pub mod handlebars;
pub mod macro_uri;
pub mod responders;

pub fn get_environment() -> String {
    dotenv::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
}

pub fn get_recaptcha_sitekey() -> &'static str {
    if get_environment() == "development" {
        RECAPTCHA_SITEKEY_FOR_TESTS
    } else {
        RECAPTCHA_CLIENT_SITEKEY
    }
}

pub fn get_recaptcha_secret() -> Result<String, dotenv::Error> {
    if get_environment() == "development" {
        Ok(RECAPTCHA_SECRET_KEY_FOR_TESTS.to_string())
    } else {
        let key = dotenv::var("RECAPTCHA_SECRET_KEY")?;

        Ok(key)
    }
}
