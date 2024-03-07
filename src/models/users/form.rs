use rocket::FromForm;

use crate::controllers::validate::{
    validate_display_name, validate_honeypot, validate_password,
    validate_password_with_confirmation,
};
use crate::models::db::enums::Gender;

#[derive(FromForm)]
pub struct RegisterFormData<'a> {
    #[field(validate = validate_display_name())]
    #[field(name = "username")]
    pub display_name: &'a str,
    #[field(validate = validate_password_with_confirmation(&self.confirm_password))]
    pub password: &'a str,
    pub confirm_password: &'a str,
    pub gender: Gender,
    #[field(validate = validate_honeypot())]
    pub honeypot: &'a str,
}

#[derive(FromForm)]
pub struct LoginFormData<'a> {
    #[field(validate = validate_display_name())]
    #[field(name = "username")]
    pub display_name: &'a str,
    #[field(validate = validate_password())]
    pub password: &'a str,
    #[field(validate = validate_honeypot())]
    pub honeypot: &'a str,
}
