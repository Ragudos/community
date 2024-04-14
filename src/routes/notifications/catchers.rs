use rocket::response::Redirect;
use rocket::{catch, Request};

use crate::auth_uri;
use crate::responders::ApiResponse;
use crate::routes::auth::login;

#[catch(401)]
pub fn unauthorized_notifications(_request: &Request<'_>) -> ApiResponse {
    ApiResponse::Redirect(Redirect::to(auth_uri!(login::login_page(_))))
}
