use rocket::{catch, Request};
use rocket_dyn_templates::{context, Template};

use crate::models::seo::metadata::SeoMetadata;
use crate::models::users::preferences::Theme;

#[catch(500)]
pub fn auth_internal_server_error_get(request: &Request<'_>) -> Template {
    let metadata = SeoMetadata::build()
        .theme(Theme::from_cookie_jar(request.cookies()))
        .title("Internal Server Error")
        .finalize();

    Template::render("pages/auth/error", context! { metadata })
}

#[catch(403)]
pub fn auth_page_forbidden(request: &Request<'_>) -> Template {
    let message = "Your request has been forbidden. This may be because of a missing CSRF-TOKEN. Please refresh the page and try again.";
    let metadata = SeoMetadata::build()
        .theme(Theme::from_cookie_jar(request.cookies()))
        .title("Forbidden")
        .finalize();

    Template::render("pages/auth/error", context! { metadata, message })
}
