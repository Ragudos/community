use rocket::{get, http::CookieJar};
use rocket_dyn_templates::{context, Template};

use crate::{controllers::users::preferences, models::seo::metadata::SeoMetadata};

#[get("/")]
pub fn page(cookie: &CookieJar<'_>) -> Template {
    let theme = preferences::get_theme_from_cookie(cookie);
    let metadata = SeoMetadata::build()
            .theme(theme)
            .finalize();

    Template::render("index", context! { metadata })
}

