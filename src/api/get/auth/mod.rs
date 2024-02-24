pub mod login;
pub mod register;

use rocket::{get, response::Redirect, uri};

use crate::models::{api::ApiResponse, users::metadata::JWT};

#[get("/<_..>")]
pub fn redirect(_jwt: JWT) -> ApiResponse {
    ApiResponse::Redirect(Redirect::to(uri!("/homepage")))
}
