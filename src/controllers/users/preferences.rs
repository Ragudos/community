use rocket::http::CookieJar;

use crate::models::users::preferences::Theme;

impl Theme {
    pub fn from_cookie_jar(cookie_jar: &CookieJar<'_>) -> Self {
        cookie_jar
            .get("theme")
            .map_or("system", |cookie| cookie.value_trimmed())
            .into()
    }
}

impl From<&str> for Theme {
    fn from(theme: &str) -> Theme {
        match theme {
            "light" => Theme::Light,
            "dark" => Theme::Dark,
            "system" => Theme::System,
            _ => Theme::System,
        }
    }
}

impl From<Theme> for &'static str {
    fn from(theme: Theme) -> &'static str {
        match theme {
            Theme::Light => "light",
            Theme::Dark => "dark",
            Theme::System => "system",
        }
    }
}
