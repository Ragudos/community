use rocket::{get, http::CookieJar};
use rocket_dyn_templates::{context, Template};

use crate::controllers::users::preferences;

#[get("/")]
pub async fn page(cookie: &CookieJar<'_>) -> Template {
    let theme = preferences::get_theme_from_cookie(cookie);

    Template::render("root", context! { theme })
}
