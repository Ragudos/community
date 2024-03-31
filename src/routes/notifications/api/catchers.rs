use rocket::http::Status;
use rocket::{catch, Request};

use crate::responders::ApiResponse;

#[catch(401)]
pub fn unauthorized_api_notifications(_request: &Request<'_>) -> ApiResponse {
    ApiResponse::Status(Status::Unauthorized)
}
