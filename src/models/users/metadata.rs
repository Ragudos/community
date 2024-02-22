use rocket::{
    serde::{Deserialize, Serialize},
    time::OffsetDateTime,
};
use sqlx::prelude::Type;

#[derive(Serialize, Deserialize, PartialEq, Eq, Type, Clone, Debug)]
#[sqlx(type_name = "gender", rename_all = "lowercase")]
pub enum Gender {
    Male,
    Female,
    Other,
    NotSpecified,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Type, Clone, Debug)]
#[sqlx(type_name = "occupation", rename_all = "lowercase")]
pub enum Occupation {
    Student,
    Teacher,
    Engineer,
    Doctor,
    Lawyer,
    Unemployed,
    Other,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Type, Clone, Debug)]
#[sqlx(type_name = "userrole", rename_all = "lowercase")]
pub enum UserRole {
    Owner,
    Admin,
    Moderator,
    User,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Type, Clone, Debug)]
#[sqlx(type_name = "referrals", rename_all = "lowercase")]
pub enum Referrals {
    Facebook,
    Twitter,
    Instagram,
    LinkedIn,
    Reddit,
    TikTok,
    Other,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Type, Clone, Debug)]
#[sqlx(type_name = "requeststatus", rename_all = "lowercase")]
pub enum RequestStatus {
    Pending,
    Accepted,
    Rejected,
    Blocked,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JWT {
    pub token: User,
    pub expires_in: OffsetDateTime,
    pub creation_date: OffsetDateTime,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub id: i32,
    pub display_name: String,
    pub display_image: Option<String>,
    pub created_at: OffsetDateTime,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserMetadata {
    pub id: i32,
    pub occupation: Occupation,
    pub gender: Gender,
    pub biography: Option<String>,
    pub is_private: bool,
    pub last_login_date: OffsetDateTime,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserSocials {
    pub id: i32,
    pub facebook: Option<String>,
    pub twitter: Option<String>,
    pub instagram: Option<String>,
    pub linkedin: Option<String>,
    pub reddit: Option<String>,
    pub tiktok: Option<String>,
    pub youtube: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserCredentials {
    pub id: i32,
    pub email: String,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserToken {
    pub user_id: i32,
    pub refresh_token: String,
    /// The time in ms that the refresh token will expire
    pub refresh_token_expires_in: OffsetDateTime,
    pub refresh_token_creation_date: OffsetDateTime,
}

