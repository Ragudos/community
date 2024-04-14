use rocket::catch;
use rocket::http::Status;

use crate::models::Toast;
use crate::responders::ApiResponse;

#[catch(401)]
pub fn unauthorized_community_api() -> ApiResponse {
    ApiResponse::Toast(
        Status::Unauthorized,
        Toast::error(Some(
            "This action requires that you are logged in.".to_string(),
        )),
    )
}

#[catch(403)]
pub fn forbidden_community_api() -> ApiResponse {
    ApiResponse::Toast(
        Status::Forbidden,
        Toast::error(Some(
            "You are forbidden to perform this action.".to_string(),
        )),
    )
}

#[catch(500)]
pub fn internal_server_error_community_api() -> ApiResponse {
    ApiResponse::Toast(
        Status::InternalServerError,
        Toast::error(Some("An internal server error occurred.".to_string())),
    )
}

#[catch(404)]
pub fn not_found_community_api() -> ApiResponse {
    ApiResponse::Toast(
        Status::NotFound,
        Toast::error(Some("The requested action could not be found. Perhaps it is yet to be implemented.".to_string())),
    )
}

#[catch(default)]
pub fn default_community_api_error() -> ApiResponse {
    ApiResponse::Toast(
        Status::InternalServerError,
        Toast::error(Some("An unknown error occurred.".to_string())),
    )
}
