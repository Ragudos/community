use serde::{Deserialize, Serialize};

pub mod api;
pub mod community;
pub mod db;
pub mod errors;
pub mod messaging;
pub mod notifications;
pub mod rate_limiter;
pub mod seo;
pub mod users;

pub struct Environment {
    pub environment: Env,
}

pub enum Env {
    Development,
    Testing,
    Production,
}

#[derive(Serialize, Deserialize)]
pub enum ToastTypes {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "info")]
    Info
}

#[derive(Serialize, Deserialize)]
pub struct Toast {
    pub message: String,
    pub r#type: Option<ToastTypes>
}

pub const JWT_NAME: &str = "Community__jwt";
pub const HOMEPAGE_COMMUNITY_LIMIT: i64 = 12;
