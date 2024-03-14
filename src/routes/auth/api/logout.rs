use rocket::delete;
use rocket::http::CookieJar;

use crate::auth_uri;
use crate::controllers::htmx::redirect::HtmxRedirect;
use crate::models::JWT_NAME;
use crate::routes::auth::login;

#[delete("/logout")]
pub fn delete(cookie_jar: &CookieJar<'_>) -> HtmxRedirect {
    cookie_jar.remove_private(JWT_NAME);
    HtmxRedirect::to(auth_uri!(login::page))
}