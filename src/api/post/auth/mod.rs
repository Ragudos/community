use rocket::{http::Status, post};

use crate::models::{api::ApiResponse, users::schema::UserJWT};

pub mod login;
pub mod register;

#[post("/<_..>")]
pub fn deny_post_request(_jwt: UserJWT) -> ApiResponse {
    ApiResponse::String(Status::Forbidden, "You are already logged in.")
}
