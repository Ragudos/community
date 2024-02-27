pub mod api;
pub mod captcha;
pub mod errors;
pub mod forms;
pub mod messaging;
pub mod notifications;
pub mod seo;
pub mod users;

pub const JWT_NAME: &str = "Community__jwt";
pub const RECAPTCHA_SITEKEY_FOR_TESTS: &str = "6LeIxAcTAAAAAJcZVRqyHh71UMIEGNQ_MXjiZKhI";
pub const RECAPTCHA_CLIENT_SITEKEY: &str = "6Lc2hH8pAAAAAC0YCMf8LsPa0O662Dw-iR-wX615";
pub const RECAPTCHA_SECRET_KEY_FOR_TESTS: &str = "6LeIxAcTAAAAAGG-vFI1TnRWxMZNFuojJ4WifJWe";
