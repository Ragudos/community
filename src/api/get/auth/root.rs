use crate::models::{
    api::ApiResponse,
    seo::metadata::SeoMetadata,
    users::{preferences::Theme, schema::UserJWT},
};
use rocket::{get, http::CookieJar};
use rocket_dyn_templates::{context, Template};

#[get("/")]
pub fn page(jwt: UserJWT, cookie: &CookieJar<'_>) -> ApiResponse {
    let theme = Theme::from_cookie_jar(cookie);
    let metadata = SeoMetadata::build().theme(theme).finalize();

    ApiResponse::Template(Template::render(
        "auth/index",
        context! {
            metadata,
            user: jwt
        },
    ))
}
