use rocket::serde::{Deserialize, Serialize};

use crate::models::db::enums::{AccountStatus, Gender, Occupation};

#[derive(Clone, Debug)]
pub struct UserTable {
    _id: i64,
    _created_at: String,
    pub uid: String,
    pub display_name: String,
    pub display_image: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FullUserInfo {
    pub uid: String,
    pub display_name: String,
    pub display_image: Option<String>,
    pub occupation: Option<Occupation>,
    pub gender: Gender,
    pub biography: Option<String>,
    pub is_private: bool,
    pub account_status: AccountStatus,
    pub facebook: Option<String>,
    pub twitter: Option<String>,
    pub instagram: Option<String>,
    pub linkedin: Option<String>,
    pub reddit: Option<String>,
    pub tiktok: Option<String>,
    pub youtube: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserJWT {
    pub uid: String,
    pub display_name: String,
    pub display_image: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserMetadata {
    pub occupation: Option<Occupation>,
    pub gender: Gender,
    pub biography: Option<String>,
    pub is_private: bool,
    pub account_status: AccountStatus,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserCredentials {
    pub email: Option<String>,
    pub password_hash: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FullName {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserSocials {
    pub facebook: Option<String>,
    pub twitter: Option<String>,
    pub instagram: Option<String>,
    pub linkedin: Option<String>,
    pub reddit: Option<String>,
    pub tiktok: Option<String>,
    pub youtube: Option<String>,
}
