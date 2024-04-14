use rocket::FromForm;

use crate::controllers::validate::{
    validate_ascii_text, validate_honeypot, validate_password,
    validate_password_with_confirmation,
};

#[derive(FromForm, Debug)]
pub struct RegisterFormData<'r> {
    #[field(validate = len(1..=60))]
    #[field(validate = validate_ascii_text())]
    #[field(name = "username")]
    pub display_name: &'r str,
    #[field(validate= len(8..=64))]
    #[field(validate = validate_password_with_confirmation(&self.confirm_password))]
    pub password: &'r str,
    pub confirm_password: &'r str,
    #[field(validate = validate_honeypot())]
    pub honeypot: &'r str,
    pub authenticity_token: &'r str,
}

#[derive(FromForm, Debug)]
pub struct LoginFormData<'r> {
    #[field(validate = len(1..=60))]
    #[field(validate = validate_ascii_text())]
    #[field(name = "username")]
    pub display_name: &'r str,
    #[field(validate = len(8..=64))]
    #[field(validate = validate_password())]
    pub password: &'r str,
    #[field(validate = validate_honeypot())]
    pub honeypot: &'r str,
    pub authenticity_token: &'r str,
}
