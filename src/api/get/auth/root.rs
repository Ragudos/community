use crate::{api::get::homepage, controllers::users::preferences::get_theme_from_cookie, homepage_uri, models::{api::ApiResponse, seo::metadata::SeoMetadata, users::metadata::JWT}};
use rocket::{get, http::CookieJar, response::Redirect};
use rocket_dyn_templates::{context, Template};
use time::{Duration, OffsetDateTime};

#[get("/")]
pub fn page(jwt: JWT, cookie: &CookieJar<'_>) -> ApiResponse {
    // This is the welcome page for authenticated users. If the JWT is older than 20 seconds, we redirect to the homepage.
    if jwt.creation_date.saturating_add(Duration::seconds(20)) < OffsetDateTime::now_utc() {
        return ApiResponse::Redirect(Redirect::to(homepage_uri!(homepage::root::page)));
    }

    let theme = get_theme_from_cookie(cookie);
    let metadata = SeoMetadata::build().theme(theme).finalize();

    ApiResponse::Template(Template::render(
        "auth/index",
        context! {
            metadata,
            user: jwt.token
        }
    ))
}
