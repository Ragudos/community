use rocket::{http::Status, post};

use crate::models::api::ApiResponse;

pub mod community;

#[post("/<_..>", rank = 2)]
pub fn deny_post_request() -> ApiResponse {
    ApiResponse::String(
        Status::Unauthorized,
        "You are not authorized to perform this action.",
    )
}
