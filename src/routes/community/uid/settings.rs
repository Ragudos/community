use rocket::get;
use rocket::http::CookieJar;
use rocket_dyn_templates::context;
use rocket_dyn_templates::Template;

use crate::controllers::htmx::IsBoosted;
use crate::models::seo::metadata::SeoMetadata;
use crate::models::users::preferences::Theme;
use crate::models::users::schema::UserJWT;

/// TODO: Community Preview here with community's uid, owner's uid, and whether the user viewing this
/// page is the owner or member of the community or not.
#[get("/<_>/settings")]
pub fn page<'r>(cookie_jar: &CookieJar<'r>, user: UserJWT, is_boosted: IsBoosted) -> Template {
    let IsBoosted(is_boosted) = is_boosted;
    let theme = Theme::from_cookie_jar(cookie_jar);
    let metadata = SeoMetadata::build().theme(theme).finalize();

    Template::render(
        "pages/community/settings",
        context! { metadata, user, is_boosted },
    )
}
