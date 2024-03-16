use rocket::get;
use rocket::http::CookieJar;
use rocket::response::Redirect;
use rocket_dyn_templates::context;
use rocket_dyn_templates::Template;

use crate::auth_uri;
use crate::controllers::htmx::redirect::HtmxRedirect;
use crate::controllers::htmx::IsBoosted;
use crate::models::api::ApiResponse;
use crate::models::seo::metadata::SeoMetadata;
use crate::models::users::preferences::Theme;
use crate::models::users::schema::UserJWT;
use crate::routes::auth::login;

// TODO: Implement to get the uid from the URL
#[get("/<_>")]
pub fn page<'r>(cookie_jar: &CookieJar<'r>, user: UserJWT, is_boosted: IsBoosted) -> Template {
    let IsBoosted(is_boosted) = is_boosted;
    let theme = Theme::from_cookie_jar(cookie_jar);
    let metadata = SeoMetadata::build().theme(theme).finalize();

    Template::render("pages/user", context! { metadata, user, is_boosted })
}

/// When the uid present does not exist, the route will be forwarded to this fn.
#[get("/<_>", rank = 2)]
pub fn page_not_found<'r>(
    cookie_jar: &CookieJar<'r>,
    user: UserJWT,
    is_boosted: IsBoosted,
) -> Template {
    let IsBoosted(is_boosted) = is_boosted;
    let theme = Theme::from_cookie_jar(cookie_jar);
    let metadata = SeoMetadata::build().theme(theme).finalize();

    Template::render("pages/user/404", context! { metadata, user, is_boosted })
}

#[get("/<_..>", rank = 2)]
pub fn logged_out(is_boosted: IsBoosted) -> ApiResponse {
    match is_boosted {
        IsBoosted(true) => ApiResponse::HtmxRedirect(HtmxRedirect::to(auth_uri!(login::page))),
        IsBoosted(false) => ApiResponse::Redirect(Redirect::to(auth_uri!(login::page))),
    }
}
