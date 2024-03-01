use rocket::{get, http::Status};

use crate::models::{api::ApiResponse, users::metadata::JWT};


#[get("/community?<search>")]
pub async fn search_community(_jwt: JWT, search: &str) -> Result<ApiResponse, ApiResponse> {
    Ok(ApiResponse::String(Status::Ok, ""))
}
