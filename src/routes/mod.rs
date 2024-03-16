pub mod auth;
pub mod comments;
pub mod community;
pub mod create;
pub mod posts;
pub mod user;

use rocket::{get, http::CookieJar};
use rocket_dyn_templates::{context, Template};

use crate::models::{
    seo::metadata::SeoMetadata,
    users::{preferences::Theme, schema::UserJWT},
};

#[get("/")]
pub fn page<'r>(cookie_jar: &CookieJar<'r>, user: Option<UserJWT>) -> Template {
    let theme = Theme::from_cookie_jar(cookie_jar);
    let metadata = SeoMetadata::build().theme(theme).finalize();

    Template::render("pages/index", context! { metadata, user })
}
