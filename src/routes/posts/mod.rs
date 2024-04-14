use rocket::get;
use rocket::http::CookieJar;
use rocket_dyn_templates::{context, Template};

use crate::controllers::htmx::IsBoosted;
use crate::models::seo::metadata::SeoMetadata;
use crate::models::users::preferences::Theme;
use crate::models::users::schema::UserJWT;

pub mod api;

#[get("/<_>/<_>")]
pub fn page<'r>(
    cookie_jar: &CookieJar<'r>,
    user: UserJWT,
    is_boosted: IsBoosted,
) -> Template {
    let IsBoosted(is_boosted) = is_boosted;
    let theme = Theme::from_cookie_jar(cookie_jar);
    let metadata = SeoMetadata::build().theme(theme).finalize();
    Template::render(
        "pages/posts/post",
        context! { metadata, user, is_boosted },
    )
}

#[get("/<_..>", rank = 4)]
pub fn logged_out(is_boosted: IsBoosted) -> Template {
    let IsBoosted(is_boosted) = is_boosted;
    Template::render("pages/forbidden", context! { is_boosted })
}

// URI path pattern: /posts/api/<community_uid>/<post_uid>
// URI path pattern: /posts/<community_uid>/<post_uid>
