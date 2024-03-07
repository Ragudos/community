use rocket::{get, http::CookieJar};
use rocket_dyn_templates::{context, Template};

use crate::models::{
    api::ApiResponse,
    db::enums::CommunityCategory,
    seo::metadata::SeoMetadata,
    users::{preferences::Theme, schema::UserJWT},
};

#[get("/?<q>&<c>&<o>")]
pub fn page(
    jwt: UserJWT,
    cookie: &CookieJar<'_>,
    q: Option<&str>,
    c: Option<&str>,
    o: Option<i64>,
) -> ApiResponse {
    let offset = match o {
        Some(offset) => offset,
        None => 0,
    };
    let theme = Theme::from_cookie_jar(cookie);
    let metadata = SeoMetadata::build().theme(theme).finalize();
    // For the buttons
    let categories = match c {
        Some(c) => c
            .split(',')
            .take(3)
            .map(|s| s.into())
            .collect::<Vec<CommunityCategory>>(),
        None => vec![],
    };

    ApiResponse::Template(Template::render(
        "homepage/index",
        context! {
            metadata,
            user: jwt,
            offset,
            search: q,
            category: c,
            categories,
        },
    ))
}
