use rocket::{http::Status, post};

use crate::models::{api::ApiResponse, users::metadata::JWT};

pub mod register;

#[post("/<_..>")]
pub fn deny_post_request(_jwt: JWT) -> ApiResponse {
    ApiResponse::String(Status::Forbidden, "You are already logged in.")
}

