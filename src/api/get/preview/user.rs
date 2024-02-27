use rocket::{get, http::Status};

use crate::models::{api::ApiResponse, users::metadata::JWT};

#[get("/user?<user_id>")]
pub async fn api_endpoint(jwt: JWT, user_id: Option<i64>) -> ApiResponse {
    ApiResponse::String(
        Status::NotImplemented,
        "This endpoint is not implemented yet.",
    )
}
