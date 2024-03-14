use rocket::response::Redirect;
use rocket::get;

use crate::auth_uri;
use crate::controllers::htmx::redirect::HtmxRedirect;
use crate::controllers::htmx::IsBoosted;
use crate::models::api::ApiResponse;
use crate::routes::auth::login;

pub mod community;
pub mod api;

#[get("/<_..>", rank = 2)]
pub fn logged_out(is_boosted: IsBoosted) -> ApiResponse {
    match is_boosted {
        IsBoosted(true) => ApiResponse::HtmxRedirect(HtmxRedirect::to(auth_uri!(login::page))),
        IsBoosted(false) => ApiResponse::Redirect(Redirect::to(auth_uri!(login::page)))
    }
}