use rocket::get;

use rocket::http::CookieJar;
use rocket_dyn_templates::context;
use rocket_dyn_templates::Template;

use crate::controllers::htmx::IsBoosted;
use crate::models::seo::metadata::SeoMetadata;
use crate::models::users::preferences::Theme;
use crate::models::users::schema::UserJWT;

pub mod api;

// When viewing a post (i.e. clicking on it, we get redirected to the post's page).
// TODO: Implement PostPreview where we check if the user is part of the community.
#[get("/<_>/<_>")]
pub fn page<'r>(cookie_jar: &CookieJar<'r>, user: UserJWT, is_boosted: IsBoosted) -> Template {
    let IsBoosted(is_boosted) = is_boosted;
    let theme = Theme::from_cookie_jar(cookie_jar);
    let metadata = SeoMetadata::build().theme(theme).finalize();
    Template::render("pages/posts/post", context! { metadata, user, is_boosted })
}

// Whether this post can be seen by a user. Since posts are in a community, we check if they're
// part of the community in a PostPreview parameter using a FromRequest guard.
#[get("/<_..>", rank = 2)]
pub fn logged_out_and_not_allowed(is_boosted: IsBoosted) -> Template {
    let IsBoosted(is_boosted) = is_boosted;
    Template::render("pages/forbidden", context! { is_boosted })
}

// URI path pattern: /posts/api/<community_uid>/<post_uid>
// URI path pattern: /posts/<community_uid>/<post_uid>
