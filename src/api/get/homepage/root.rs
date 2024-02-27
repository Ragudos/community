use rocket::{get, http::CookieJar};
use rocket_dyn_templates::{context, Template};

use crate::{
    controllers::users::preferences::get_theme_from_cookie,
    models::{api::ApiResponse, seo::metadata::SeoMetadata, users::metadata::JWT},
};

#[get("/")]
pub fn page(jwt: JWT, cookie: &CookieJar<'_>) -> ApiResponse {
    let theme = get_theme_from_cookie(cookie);
    let metadata = SeoMetadata::build().theme(theme).finalize();

    ApiResponse::Template(Template::render(
        "homepage/index",
        context! {
            metadata,
            user: jwt.token
        },
    ))
}
