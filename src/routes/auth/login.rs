use rocket::get;
use rocket::http::{CookieJar, Status};
use rocket_csrf_token::CsrfToken;
use rocket_dyn_templates::{context, Template};

use crate::controllers::htmx::IsBoosted;
use crate::models::seo::metadata::SeoMetadata;
use crate::models::users::preferences::Theme;
use crate::responders::ApiResponse;

#[get("/login?<isredirected>", rank = 2)]
pub fn login_page<'r>(
    cookie_jar: &CookieJar<'r>,
    is_boosted: IsBoosted,
    isredirected: Option<bool>,
    csrf_token: CsrfToken,
) -> Result<ApiResponse, ApiResponse> {
    let IsBoosted(is_boosted) = is_boosted;
    let theme = Theme::from_cookie_jar(cookie_jar);
    let metadata = SeoMetadata::build()
        .theme(theme)
        .title("Sign In to Community")
        .finalize();
    let authenticity_token =
        csrf_token.authenticity_token().map_err(|error| {
            eprintln!("Error generating authenticity token: {:?}", error);
            return ApiResponse::Status(Status::InternalServerError);
        })?;

    Ok(ApiResponse::Render {
        status: Status::Ok,
        template: Some(Template::render(
            "pages/auth/login",
            context! { metadata, is_boosted, isredirected, authenticity_token },
        )),
        headers: None,
    })
}
