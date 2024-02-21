use std::convert::Infallible;

use rocket::{
    async_trait,
    http::Status,
    outcome::IntoOutcome,
    request::{FromRequest, Outcome, Request},
    serde::{Deserialize, Serialize},
    time::OffsetDateTime,
};
use serde_json::from_str;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Gender {
    Male,
    Female,
    Other,
    NotSpecified,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Occupation {
    Student,
    Teacher,
    Engineer,
    Doctor,
    Lawyer,
    Unemployed,
    Other,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum UserRole {
    Owner,
    Admin,
    Moderator,
    User,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Referrals {
    Facebook,
    Twitter,
    Instagram,
    LinkedIn,
    Reddit,
    TikTok,
    Other,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum RequestStatus {
    Pending,
    Accepted,
    Rejected,
    Blocked,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub id: isize,
    pub display_name: String,
    pub display_image: Option<String>,
    pub created_at: OffsetDateTime,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserMetadata<'lifetime> {
    pub id: isize,
    pub occupation: Occupation,
    pub gender: Gender,
    pub biography: Option<&'lifetime str>,
    pub is_private: bool,
    pub last_login_date: OffsetDateTime,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserSocials<'lifetime> {
    pub id: isize,
    pub facebook: Option<&'lifetime str>,
    pub twitter: Option<&'lifetime str>,
    pub instagram: Option<&'lifetime str>,
    pub linkedin: Option<&'lifetime str>,
    pub reddit: Option<&'lifetime str>,
    pub tiktok: Option<&'lifetime str>,
    pub youtube: Option<&'lifetime str>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserCredentials<'lifetime> {
    pub id: isize,
    pub email: &'lifetime str,
    pub password_hash: &'lifetime str,
    pub first_name: &'lifetime str,
    pub last_name: &'lifetime str,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserToken<'lifetime> {
    pub id: isize,
    pub refresh_token: &'lifetime str,
    /// The time in ms that the refresh token will expire
    pub refresh_token_expires_in: isize,
    pub refresh_token_creation_date: OffsetDateTime,
}

#[async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = Infallible;

    async fn from_request(request: &'r Request<'_>) -> Outcome<User, Self::Error> {
        request
            .cookies()
            .get_private("Community__user-metadata")
            .and_then(|cookie| {
                let user_str = cookie.value_trimmed();
                let parsed_user: Option<User> = from_str(user_str).ok();

                match parsed_user {
                    Some(user) => {
                        // #TODO check if the user_metadata is expired. If so, check refresh token.
                        // If refresh token is expired, return None
                        Some(user)
                    }
                    None => {
                        request.cookies().remove_private(cookie);
                        None
                    }
                }
            })
            .or_forward(Status::Unauthorized)
    }
}
