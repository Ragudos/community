use std::str::FromStr;

use rocket::{request::FromParam, FromForm};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

pub mod api;
pub mod community;
pub mod db;
pub mod errors;
pub mod messaging;
pub mod notifications;
pub mod query;
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
    Info,
}

#[derive(Serialize, Deserialize)]
pub struct Toast {
    pub message: String,
    pub r#type: Option<ToastTypes>,
}

#[derive(Debug, Clone)]
pub struct StringUuid(pub Uuid);

impl<'a> FromParam<'a> for StringUuid {
    type Error = &'a str;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        let uid = Uuid::from_str(param).map_err(|_| "Invalid UUID")?;

        Ok(StringUuid(uid))
    }
}

pub const JWT_NAME: &str = "Community__jwt";
pub const HOMEPAGE_COMMUNITY_LIMIT: i64 = 12;
pub const COMMUNITY_POST_LIMIT: i64 = 20;
