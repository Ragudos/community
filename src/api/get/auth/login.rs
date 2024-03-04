use rocket::{get, http::CookieJar};
use rocket_dyn_templates::{context, Template};

use crate::{
    controllers::users::preferences::get_theme_from_cookie,
    helpers::get_recaptcha_sitekey,
    models::{api::ApiResponse, captcha::Captcha, seo::metadata::SeoMetadata},
};

#[get("/login", rank = 2)]
pub fn page(cookie: &CookieJar<'_>) -> ApiResponse {
    let theme = get_theme_from_cookie(cookie);
    let metadata = SeoMetadata::build().theme(theme).finalize();

    ApiResponse::Template(Template::render(
        "auth/login",
        context! {
            metadata,
            captcha: Captcha {
                action: "login",
                sitekey: get_recaptcha_sitekey()
            }
        },
    ))
}
