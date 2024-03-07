use rocket::{delete, http::CookieJar};

use crate::{
    api::get::auth,
    auth_uri,
    controllers::htmx::redirect::HtmxRedirect,
    models::{api::ApiResponse, users::schema::UserJWT, JWT_NAME},
};

#[delete("/logout")]
pub async fn api_endpoint(_jwt: UserJWT, cookie_jar: &CookieJar<'_>) -> ApiResponse {
    cookie_jar.remove_private(JWT_NAME);
    ApiResponse::HtmxRedirect(HtmxRedirect::to(auth_uri!(auth::login::page)))
}

#[delete("/logout", rank = 2)]
pub async fn deny_delete_request() -> ApiResponse {
    ApiResponse::HtmxRedirect(HtmxRedirect::to(auth_uri!(auth::login::page)))
}
