use rocket::get;
use rocket::http::CookieJar;
use rocket::FromForm;
use rocket_dyn_templates::context;
use rocket_dyn_templates::Template;

use crate::controllers::htmx::IsBoosted;
use crate::models::db::enums::UserRole;
use crate::models::seo::metadata::SeoMetadata;
use crate::models::users::preferences::Theme;
use crate::models::users::schema::UserJWT;

#[derive(FromForm)]
pub struct MemberFilter {
    pub t: Option<UserRole>,
}

/// TODO: Community Preview here with community's uid, owner's uid, and whether the user viewing this
/// page is the owner or member of the community or not.
/// Also do the t filter which stands for type, and it can be either `members`, `admins`, `online`.
#[get("/<community_id>/members?<includeheader>&<t..>")]
pub fn page<'r>(
    cookie_jar: &CookieJar<'r>,
    user: UserJWT,
    is_boosted: IsBoosted,
    includeheader: Option<bool>,
    t: Option<UserRole>,
    community_id: i64,
) -> Template {
    let IsBoosted(is_boosted) = is_boosted;
    let theme = Theme::from_cookie_jar(cookie_jar);
    let metadata = SeoMetadata::build().theme(theme).finalize();

    Template::render(
        "pages/community/members",
        context! { metadata, user, is_boosted, includeheader, community_id, current_page: "members" },
    )
}
