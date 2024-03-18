use rocket::get;
use rocket::http::CookieJar;
use rocket::response::Redirect;
use rocket_dyn_templates::context;
use rocket_dyn_templates::Template;

use crate::auth_uri;
use crate::controllers::htmx::redirect::HtmxRedirect;
use crate::controllers::htmx::IsBoosted;
use crate::models::query::ListQuery;
use crate::models::seo::metadata::SeoMetadata;
use crate::models::users::preferences::Theme;
use crate::models::users::schema::UserJWT;
use crate::responders::ApiResponse;
use crate::routes::auth::login;

pub mod api;
pub mod uid;

// We don't do anything with the query here since
// this will just transfer over the initial query of the URI
// to the HTML markup for HTMX to handle the AJAX side of things.
#[get("/?<list_query..>")]
pub fn page<'r>(
    cookie_jar: &CookieJar<'r>,
    user: UserJWT,
    is_boosted: IsBoosted,
    list_query: Option<ListQuery<'r>>,
) -> Template {
    let IsBoosted(is_boosted) = is_boosted;
    let theme = Theme::from_cookie_jar(cookie_jar);
    let metadata = SeoMetadata::build().theme(theme).finalize();

    Template::render(
        "pages/community",
        context! { metadata, user, is_boosted, list_query },
    )
}

#[get("/<_..>", rank = 3)]
pub fn logged_out(is_boosted: IsBoosted) -> ApiResponse {
    match is_boosted {
        IsBoosted(true) => ApiResponse::HtmxRedirect(HtmxRedirect::to(auth_uri!(login::page))),
        IsBoosted(false) => ApiResponse::Redirect(Redirect::to(auth_uri!(login::page))),
    }
}
