pub mod login;
pub mod register;
pub mod root;

use rocket::{get, response::Redirect};

use crate::{
    api::get::auth::login as login_page,
    auth_uri,
    models::{api::ApiResponse, users::schema::UserJWT},
};

#[get("/<_..>")]
pub fn redirect(_jwt: UserJWT) -> ApiResponse {
    ApiResponse::Redirect(Redirect::to("/homepage"))
}

#[get("/", rank = 2)]
pub fn deny_welcome_page() -> ApiResponse {
    ApiResponse::Redirect(Redirect::to(auth_uri!(login_page::page)))
}
