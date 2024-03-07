use rocket::{get, http::CookieJar};
use rocket_dyn_templates::{context, Template};

use crate::models::{api::ApiResponse, seo::metadata::SeoMetadata, users::preferences::Theme};

#[get("/")]
pub fn page(cookie: &CookieJar<'_>) -> ApiResponse {
    let theme = Theme::from_cookie_jar(cookie);
    let metadata = SeoMetadata::build().theme(theme).finalize();

    ApiResponse::Template(Template::render("index", context! { metadata }))
}
