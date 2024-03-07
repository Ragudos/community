pub mod db;
pub mod handlebars;
pub mod macro_uri;
pub mod responders;

pub fn get_environment() -> String {
    dotenv::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
}
