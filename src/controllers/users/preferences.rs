use rocket::http::CookieJar;

use crate::models::users::preferences::Theme;

pub fn get_theme_from_cookie(cookie: &CookieJar<'_>) -> Theme {
    if let Some(cookie) = cookie.get("theme") {
        cookie.value().into()
    } else {
        Theme::System
    }
}
