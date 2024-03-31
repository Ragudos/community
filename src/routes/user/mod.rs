use rocket::get;
use rocket::http::{CookieJar, Status};
use rocket::response::Redirect;
use rocket_dyn_templates::{context, Template};

use crate::auth_uri;
use crate::controllers::htmx::IsBoosted;
use crate::models::seo::metadata::SeoMetadata;
use crate::models::users::preferences::Theme;
use crate::models::users::schema::UserJWT;
use crate::responders::ApiResponse;
use crate::routes::auth::login;

pub mod api;
pub mod catchers;

#[get("/<id>")]
pub fn page<'r>(
    cookie_jar: &CookieJar<'r>,
    user: UserJWT,
    is_boosted: IsBoosted,
    id: i64,
) -> Template {
    let IsBoosted(is_boosted) = is_boosted;
    let theme = Theme::from_cookie_jar(cookie_jar);
    let metadata = SeoMetadata::build().theme(theme).finalize();

    Template::render("pages/user", context! { metadata, user, is_boosted, id })
}

#[get("/<_..>", rank = 3)]
pub fn malformed_uid(
    cookie_jar: &CookieJar<'_>,
    user: UserJWT,
    is_boosted: IsBoosted,
) -> ApiResponse {
    let theme = Theme::from_cookie_jar(cookie_jar);
    let metadata = SeoMetadata::build().theme(theme).finalize();
    let IsBoosted(is_boosted) = is_boosted;

    ApiResponse::Render {
        status: Status::BadRequest,
        template: Some(Template::render(
            "partials/user/invalid_uid",
            context! {
                user, is_boosted, metadata
            },
        )),
        headers: None,
    }
}

/// Just a no content for any request made where the first
/// endpoint has forwarded.
#[get("/<_..>", rank = 4)]
pub fn logged_out() -> ApiResponse {
    ApiResponse::Redirect(Redirect::to(auth_uri!(login::login_page(Some(
        true
    )))))
}
