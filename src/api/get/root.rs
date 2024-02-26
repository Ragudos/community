use rocket::{get, http::CookieJar};
use rocket_dyn_templates::{context, Template};

use crate::{
    controllers::users::preferences,
    models::{api::ApiResponse, seo::metadata::SeoMetadata},
};

#[get("/")]
pub fn page(cookie: &CookieJar<'_>) -> ApiResponse {
    let theme = preferences::get_theme_from_cookie(cookie);
    let metadata = SeoMetadata::build().theme(theme).finalize();

    ApiResponse::Template(Template::render("index", context! { metadata }))
}
