use crate::models::{
    api::ApiResponse,
    seo::metadata::SeoMetadata,
    users::{preferences::Theme, schema::UserJWT},
};
use rocket::get;
use rocket::http::CookieJar;
use rocket_dyn_templates::{context, Template};

#[get("/community")]
pub fn page(jwt: UserJWT, cookie_jar: &CookieJar<'_>) -> ApiResponse {
    let theme = Theme::from_cookie_jar(cookie_jar);
    let metadata = SeoMetadata::build().theme(theme).finalize();

    ApiResponse::Template(Template::render(
        "create/community",
        context! {
            metadata,
            user: jwt,
        },
    ))
}
