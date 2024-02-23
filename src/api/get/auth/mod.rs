pub mod register;
pub mod login;

use rocket::{uri, get, response::Redirect};

use crate::models::{api::ApiResponse, users::metadata::JWT};

#[get("/<_..>")]
pub fn redirect(_jwt: JWT) -> ApiResponse {
    ApiResponse::Redirect(Redirect::to(uri!("/homepage")))
}

