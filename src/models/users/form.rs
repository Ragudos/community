use rocket::FromForm;

use crate::controllers::validate::{
    validate_ascii_text, validate_honeypot, validate_password, validate_password_with_confirmation,
};

#[derive(FromForm, Debug)]
pub struct RegisterFormData {
    #[field(validate = len(1..=60))]
    #[field(validate = validate_ascii_text())]
    #[field(name = "username")]
    pub display_name: String,
    #[field(validate= len(8..=64))]
    #[field(validate = validate_password_with_confirmation(&self.confirm_password))]
    pub password: String,
    pub confirm_password: String,
    #[field(validate = validate_honeypot())]
    pub honeypot: String,
}

#[derive(FromForm, Debug)]
pub struct LoginFormData {
    #[field(validate = len(1..=60))]
    #[field(validate = validate_ascii_text())]
    #[field(name = "username")]
    pub display_name: String,
    #[field(validate = len(8..=64))]
    #[field(validate = validate_password())]
    pub password: String,
    #[field(validate = validate_honeypot())]
    pub honeypot: String,
}
