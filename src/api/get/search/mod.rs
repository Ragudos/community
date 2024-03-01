use rocket::{get, http::Status};

use crate::models::api::ApiResponse;

pub mod community;

#[get("/<_..>")]
pub async fn deny_search() -> ApiResponse {
    ApiResponse::String(Status::Unauthorized, "Unauthorized")
}
