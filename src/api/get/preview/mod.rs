use rocket::{get, http::Status};

use crate::models::api::ApiResponse;

pub mod community;
pub mod user;

// Previews of a user, community, etc.
// Their non-sensitive data.

#[get("/<_..>", rank = 2)]
pub fn deny_request() -> ApiResponse {
    ApiResponse::String(
        Status::Forbidden,
        "You are not allowed to access this resource.",
    )
}
