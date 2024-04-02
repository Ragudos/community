use rocket::http::Status;
use rocket::{catch, Request};

use crate::models::Toast;
use crate::responders::ApiResponse;

#[catch(401)]
pub fn unauthorized_api_notifications(_request: &Request<'_>) -> ApiResponse {
    ApiResponse::Toast(
        Status::Unauthorized,
        Toast::error(Some(
            "You have been logged out. Please log in again.".to_string(),
        )),
    )
}

#[catch(403)]
pub fn forbidden_api_notifications(_request: &Request<'_>) -> ApiResponse {
    ApiResponse::Toast(
        Status::Unauthorized,
        Toast::error(Some(
            "You are forbidden to perform this action.".to_string(),
        )),
    )
}

#[catch(500)]
pub fn internal_server_error_api_notifications(
    _request: &Request<'_>,
) -> ApiResponse {
    ApiResponse::Toast(
        Status::InternalServerError,
        Toast::error(Some(
            "Something went wrong. Please try again later.".to_string(),
        )),
    )
}
