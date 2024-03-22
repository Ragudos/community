use rocket::delete;
use rocket::http::CookieJar;
use rocket::response::Redirect;

use crate::auth_uri;
use crate::models::JWT_NAME;
use crate::routes::auth::login;

#[delete("/logout")]
pub fn delete(cookie_jar: &CookieJar<'_>) -> Redirect {
    cookie_jar.remove_private(JWT_NAME);
    Redirect::to(auth_uri!(login::page))
}
