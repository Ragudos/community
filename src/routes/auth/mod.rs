use rocket::get;
use rocket::response::Redirect;

use crate::controllers::htmx::redirect::HtmxRedirect;
use crate::controllers::htmx::IsBoosted;
use crate::models::api::ApiResponse;
use crate::models::query::ListQuery;
use crate::models::users::schema::UserJWT;
use crate::routes::community;
use crate::{auth_uri, community_uri};

pub mod api;
pub mod login;
pub mod register;

#[get("/<_..>")]
pub fn logged_in(_user: UserJWT, is_boosted: IsBoosted) -> ApiResponse {
    match is_boosted {
        IsBoosted(true) => {
            ApiResponse::HtmxRedirect(HtmxRedirect::to(community_uri!(community::page(_))))
        }
        IsBoosted(false) => ApiResponse::Redirect(Redirect::to(community_uri!(community::page(_)))),
    }
}

#[get("/", rank = 2)]
pub fn logged_out(is_boosted: IsBoosted) -> ApiResponse {
    match is_boosted {
        IsBoosted(true) => ApiResponse::HtmxRedirect(HtmxRedirect::to(auth_uri!(login::page))),
        IsBoosted(false) => ApiResponse::Redirect(Redirect::to(auth_uri!(login::page))),
    }
}
