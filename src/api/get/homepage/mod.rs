use rocket::{get, response::Redirect};

use crate::{auth_uri, models::api::ApiResponse, api::get::auth::login};

pub mod root;

#[get("/<_..>", rank = 2)]
pub fn redirect() -> ApiResponse {
    ApiResponse::Redirect(Redirect::to(auth_uri!(login::page)))
}