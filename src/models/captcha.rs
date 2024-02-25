use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Captcha {
    pub action: &'static str    
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CaptchaToken<'lifetime> {
    pub token: &'lifetime str
}
