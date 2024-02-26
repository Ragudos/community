pub mod login;
pub mod register;
pub mod root;

use rocket::{get, response::Redirect};

use crate::{
    api::get::homepage::root as index_route, homepage_uri, models::{api::ApiResponse, users::metadata::JWT}
};

#[get("/<_..>")]
pub fn redirect(_jwt: JWT) -> ApiResponse {
    ApiResponse::Redirect(Redirect::to(homepage_uri!(index_route::page)))
}
