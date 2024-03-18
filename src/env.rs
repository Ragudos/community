pub enum Env {
    Development,
    Testing,
    Production,
}

pub struct Environment {
    pub environment: Env,
}

impl From<String> for Env {
    fn from(env: String) -> Self {
        match env.as_str() {
            "development" => Self::Development,
            "testing" => Self::Testing,
            "production" => Self::Production,
            _ => panic!("Invalid environment"),
        }
    }
}
