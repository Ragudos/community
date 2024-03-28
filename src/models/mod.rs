use serde::{Deserialize, Serialize};

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

impl Toast {
    pub fn success(message: Option<String>) -> Self {
        Self {
            message: message.unwrap_or("Success".to_string()),
            r#type: Some(ToastTypes::Success),
        }
    }

    pub fn error(message: Option<String>) -> Self {
        Self {
            message: message.unwrap_or("Something went wrong".to_string()),
            r#type: Some(ToastTypes::Error),
        }
    }

    pub fn warning(message: String) -> Self {
        Self {
            message,
            r#type: Some(ToastTypes::Warning),
        }
    }

    pub fn info(message: String) -> Self {
        Self {
            message,
            r#type: Some(ToastTypes::Info),
        }
    }

    pub fn message(message: String) -> Self {
        Self {
            message,
            r#type: None,
        }
    }
}

pub const JWT_NAME: &str = "CommunityAuthSession__jwt";
pub const HOMEPAGE_COMMUNITY_LIMIT: i64 = 12;
pub const COMMUNITY_POST_LIMIT: i64 = 20;
