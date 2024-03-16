use rocket::get;
use rocket::http::CookieJar;
use rocket_dyn_templates::context;
use rocket_dyn_templates::Template;

use crate::controllers::htmx::IsBoosted;
use crate::models::seo::metadata::SeoMetadata;
use crate::models::users::preferences::Theme;

#[get("/register", rank = 2)]
pub fn page<'r>(cookie_jar: &CookieJar<'r>, is_boosted: IsBoosted) -> Template {
    let IsBoosted(is_boosted) = is_boosted;
    let theme = Theme::from_cookie_jar(cookie_jar);
    let metadata = SeoMetadata::build().theme(theme).finalize();

    Template::render("pages/auth/register", context! { metadata, is_boosted })
}
