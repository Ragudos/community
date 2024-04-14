use rocket::get;
use rocket::response::Redirect;

use crate::controllers::htmx::redirect::HtmxRedirect;
use crate::controllers::htmx::IsBoosted;
use crate::models::query::ListQuery;
use crate::models::users::schema::UserJWT;
use crate::responders::ApiResponse;
use crate::routes::discover;
use crate::{auth_uri, discover_uri};

pub mod api;
pub mod catchers;
pub mod login;
pub mod register;

#[get("/")]
pub fn logged_in(_user: UserJWT, is_boosted: IsBoosted) -> ApiResponse {
    match is_boosted {
        IsBoosted(true) => ApiResponse::HtmxRedirect(HtmxRedirect::to(
            discover_uri!(discover::discover_page(Some(true), _)),
        )),
        IsBoosted(false) => ApiResponse::Redirect(Redirect::to(discover_uri!(
            discover::discover_page(Some(true), _)
        ))),
    }
}

#[get("/login")]
pub fn redirect_login(_user: UserJWT, is_boosted: IsBoosted) -> ApiResponse {
    match is_boosted {
        IsBoosted(true) => ApiResponse::HtmxRedirect(HtmxRedirect::to(
            discover_uri!(discover::discover_page(Some(true), _)),
        )),
        IsBoosted(false) => ApiResponse::Redirect(Redirect::to(discover_uri!(
            discover::discover_page(Some(true), _)
        ))),
    }
}

#[get("/register")]
pub fn redirect_register(_user: UserJWT, is_boosted: IsBoosted) -> ApiResponse {
    match is_boosted {
        IsBoosted(true) => ApiResponse::HtmxRedirect(HtmxRedirect::to(
            discover_uri!(discover::discover_page(Some(true), _)),
        )),
        IsBoosted(false) => ApiResponse::Redirect(Redirect::to(discover_uri!(
            discover::discover_page(Some(true), _)
        ))),
    }
}

#[get("/", rank = 2)]
pub fn logged_out() -> ApiResponse {
    ApiResponse::Redirect(Redirect::to(auth_uri!(login::login_page(_))))
}
