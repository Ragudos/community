use rocket::{get, http::CookieJar};
use rocket_dyn_templates::{context, Template};

use crate::{
    controllers::users::preferences::get_theme_from_cookie, models::seo::metadata::SeoMetadata,
};

#[get("/login", rank = 2)]
pub fn page(cookie: &CookieJar<'_>) -> Template {
    let theme = get_theme_from_cookie(cookie);
    let metadata = SeoMetadata::build().theme(theme).finalize();

    Template::render("auth/login", context! { metadata })
}
