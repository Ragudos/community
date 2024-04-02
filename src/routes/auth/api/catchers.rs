use rocket::catch;
use rocket::http::Status;
use rocket_dyn_templates::{context, Template};

use crate::models::Toast;
use crate::responders::ApiResponse;

#[catch(500)]
pub fn auth_api_internal_server_error() -> ApiResponse {
    ApiResponse::Toast(
        Status::InternalServerError,
        Toast::error(Some(
            "An internal server error occurred. Please try again later."
                .to_string(),
        )),
    )
}

#[catch(403)]
pub fn forbidden_auth_api() -> ApiResponse {
    ApiResponse::Toast(
        Status::Forbidden,
        Toast::error(Some("Your request has been forbidden. This may be because of a missing CSRF-TOKEN. Please refresh the page and try again.".to_string()))
    )
}

#[catch(404)]
pub fn not_found_auth_api() -> ApiResponse {
    ApiResponse::Toast(
        Status::NotFound,
        Toast::error(Some("The requested action could not be found. Perhaps it is yet to be implemented".to_string()))
    )
}

#[catch(default)]
pub fn default_auth_api_error() -> ApiResponse {
    ApiResponse::Toast(
        Status::InternalServerError,
        Toast::error(Some("An unknown error occurred".to_string())),
    )
}
