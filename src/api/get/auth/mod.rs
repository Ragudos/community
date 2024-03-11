pub mod login;
pub mod register;

use rocket::{get, response::Redirect};

use crate::models::{api::ApiResponse, users::schema::UserJWT};

#[get("/<_..>")]
pub fn redirect(_jwt: UserJWT) -> ApiResponse {
    ApiResponse::Redirect(Redirect::to("/homepage"))
}
