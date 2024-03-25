use rocket::get;
use rocket::response::Redirect;

use crate::auth_uri;
use crate::responders::ApiResponse;
use crate::routes::auth::login;

pub mod api;
pub mod community;

#[get("/<_..>", rank = 3)]
pub fn logged_out() -> ApiResponse {
    ApiResponse::Redirect(Redirect::to(auth_uri!(login::page(Some(true)))))
}
