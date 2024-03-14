use rocket::response::Redirect;
use rocket_dyn_templates::Metadata;
use rocket::post;

use crate::auth_uri;
use crate::controllers::htmx::redirect::HtmxRedirect;
use crate::models::api::ApiResponse;
use crate::controllers::htmx::IsHTMX;
use crate::routes::auth::login;

pub mod community;

#[post("/<_..>", rank = 2)]
pub fn logged_out<'r>(template_metadata: Metadata<'r>, is_htmx: IsHTMX) -> ApiResponse {
    match is_htmx {
        IsHTMX(true) => ApiResponse::HtmxRedirect(HtmxRedirect::to(auth_uri!(login::page))),
        IsHTMX(false) => ApiResponse::Redirect(Redirect::to(auth_uri!(login::page)))
    }
} 
