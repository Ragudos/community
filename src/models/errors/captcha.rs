use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum RecaptchaErrorCodes {
    MissingSecret,
    InvalidSecret,
    MissingResponse,
    InvalidResponse,
    BadRequest,
    TimeoutOrDuplicate,
}

impl From<String> for RecaptchaErrorCodes {
    fn from(s: String) -> Self {
        match s.as_str() {
            "missing-input-secret" => RecaptchaErrorCodes::MissingSecret,
            "invalid-input-secret" => RecaptchaErrorCodes::InvalidSecret,
            "missing-input-response" => RecaptchaErrorCodes::MissingResponse,
            "invalid-input-response" => RecaptchaErrorCodes::InvalidResponse,
            "bad-request" => RecaptchaErrorCodes::BadRequest,
            "timeout-or-duplicate" => RecaptchaErrorCodes::TimeoutOrDuplicate,
            _ => RecaptchaErrorCodes::BadRequest,
        }
    }
}

impl From<RecaptchaErrorCodes> for String {
    fn from(e: RecaptchaErrorCodes) -> Self {
        match e {
            RecaptchaErrorCodes::MissingSecret => "missing-input-secret".to_string(),
            RecaptchaErrorCodes::InvalidSecret => "invalid-input-secret".to_string(),
            RecaptchaErrorCodes::MissingResponse => "missing-input-response".to_string(),
            RecaptchaErrorCodes::InvalidResponse => "invalid-input-response".to_string(),
            RecaptchaErrorCodes::BadRequest => "bad-request".to_string(),
            RecaptchaErrorCodes::TimeoutOrDuplicate => "timeout-or-duplicate".to_string(),
        }
    }
}
