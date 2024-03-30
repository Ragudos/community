use rocket::catch;
use rocket::http::Status;
use rocket::Request;

use crate::responders::ApiResponse;

#[catch(401)]
pub fn unauthorized_notifications(request: &Request<'_>) -> ApiResponse {
    ApiResponse::Status(Status::Unauthorized)
}
