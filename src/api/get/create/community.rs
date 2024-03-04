use crate::{
    controllers::users::preferences::get_theme_from_cookie, helpers::get_recaptcha_sitekey, models::{api::ApiResponse, captcha::Captcha, seo::metadata::SeoMetadata, users::metadata::JWT}
};
use rocket::get;
use rocket::http::CookieJar;
use rocket_dyn_templates::{context, Template};

#[get("/community")]
pub fn page(jwt: JWT, cookie_jar: &CookieJar<'_>) -> ApiResponse {
    let theme = get_theme_from_cookie(cookie_jar);
    let metadata = SeoMetadata::build().theme(theme).finalize();

    ApiResponse::Template(Template::render(
        "create/community",
        context! {
            metadata,
            user: jwt.token,
            captcha: Captcha {
                action: "create_community",
                sitekey: get_recaptcha_sitekey()
            }
        },
    ))
}
