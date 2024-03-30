use rocket::get;
use rocket::http::CookieJar;
use rocket::http::Status;
use rocket_csrf_token::CsrfToken;
use rocket_dyn_templates::context;
use rocket_dyn_templates::Template;

use crate::controllers::htmx::IsBoosted;
use crate::models::seo::metadata::SeoMetadata;
use crate::models::users::preferences::Theme;
use crate::models::users::schema::UserJWT;
use crate::responders::ApiResponse;

#[get("/community")]
pub fn community_page<'r>(
    cookie_jar: &CookieJar<'r>,
    user: UserJWT,
    is_boosted: IsBoosted,
    csrf_token: CsrfToken,
) -> Result<ApiResponse, ApiResponse> {
    let IsBoosted(is_boosted) = is_boosted;
    let theme = Theme::from_cookie_jar(cookie_jar);
    let metadata = SeoMetadata::build().theme(theme).finalize();
    let authenticity_token = csrf_token.authenticity_token().map_err(|_| {
        return ApiResponse::Status(Status::InternalServerError);
    })?;

    Ok(ApiResponse::Render {
        status: Status::Ok,
        template: Some(Template::render(
            "pages/create/community",
            context! { metadata, user, is_boosted },
        )),
        headers: None,
    })
}
