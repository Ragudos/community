use std::str::FromStr;

use rocket::request::FromParam;
use rocket_dyn_templates::Template;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

use crate::responders::ApiResponse;

pub mod community;
pub mod db;
pub mod errors;
pub mod messaging;
pub mod notifications;
pub mod query;
pub mod seo;
pub mod users;

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
    type Error = ApiResponse;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        Ok(StringUuid(Uuid::from_str(param)?))
    }
}

pub const JWT_NAME: &str = "Community__jwt";
pub const HOMEPAGE_COMMUNITY_LIMIT: i64 = 12;
pub const COMMUNITY_POST_LIMIT: i64 = 20;
