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

impl Into<&str> for Theme {
    fn into(self) -> &'static str {
        match self {
            Theme::Light => "light",
            Theme::Dark => "dark",
            Theme::System => "system",
        }
    }
}

