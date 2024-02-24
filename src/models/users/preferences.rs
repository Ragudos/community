use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Theme {
    Light,
    Dark,
    System,
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

