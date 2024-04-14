use rocket::get;
use rocket::http::CookieJar;
use rocket_dyn_templates::{context, Template};

use crate::controllers::htmx::IsBoosted;
use crate::models::seo::metadata::SeoMetadata;
use crate::models::users::preferences::Theme;
use crate::models::users::schema::UserJWT;

pub mod api;
pub mod catchers;

#[get("/<id>")]
pub fn user_profile_page<'r>(
    cookie_jar: &CookieJar<'r>,
    user: UserJWT,
    is_boosted: IsBoosted,
    id: i64,
) -> Template {
    let IsBoosted(is_boosted) = is_boosted;
    let theme = Theme::from_cookie_jar(cookie_jar);
    let metadata = SeoMetadata::build().theme(theme).finalize();

    Template::render("pages/user", context! { metadata, user, is_boosted, id })
}
