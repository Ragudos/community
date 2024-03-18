use rocket::FromForm;

use crate::controllers::validate::{
    validate_ascii_text, validate_honeypot, validate_password, validate_password_with_confirmation,
};

#[derive(FromForm, Debug)]
pub struct RegisterFormData<'a> {
    #[field(validate = len(1..=60))]
    #[field(validate = validate_ascii_text())]
    #[field(name = "username")]
    pub display_name: &'a str,
    #[field(validate= len(8..=64))]
    #[field(validate = validate_password_with_confirmation(&self.confirm_password))]
    pub password: &'a str,
    pub confirm_password: &'a str,
    #[field(validate = validate_honeypot())]
    pub honeypot: &'a str,
}

#[derive(FromForm, Debug)]
pub struct LoginFormData<'a> {
    #[field(validate = len(1..=60))]
    #[field(validate = validate_ascii_text())]
    #[field(name = "username")]
    pub display_name: &'a str,
    #[field(validate = len(8..=64))]
    #[field(validate = validate_password())]
    pub password: &'a str,
    #[field(validate = validate_honeypot())]
    pub honeypot: &'a str,
}
