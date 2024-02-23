use rocket::{get, http::CookieJar};
use rocket_dyn_templates::Template;

use crate::{controllers::users::preferences::get_theme_from_cookie, models::seo::metadata::SeoMetadata};

#[get("/register", rank = 2)]
pub fn page(cookie: &CookieJar<'_>) -> Template {
    let theme = get_theme_from_cookie(cookie);
    let metadata = SeoMetadata::build()
            .theme(theme)
            .finalize();

    Template::render("auth/register", metadata)
}

