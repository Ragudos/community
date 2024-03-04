use rocket::get;
use rocket::response::Redirect;

use crate::{api::get::auth::login, auth_uri, models::api::ApiResponse};

pub mod community;

#[get("/<_..>", rank = 2)]
pub fn redirect() -> ApiResponse {
    ApiResponse::Redirect(Redirect::to(auth_uri!(login::page)))
}
