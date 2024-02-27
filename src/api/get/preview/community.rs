use rocket::{get, http::Status};

use crate::models::{api::ApiResponse, users::metadata::JWT};

/// offset is how much the database should offset the results by.
#[get("/community?<offset>")]
pub async fn api_endpoint(jwt: JWT, offset: Option<i64>) -> ApiResponse {
    ApiResponse::String(
        Status::NotImplemented,
        "This endpoint is not implemented yet.",
    )
}
