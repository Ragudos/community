use rocket::FromForm;

use crate::{controllers::forms::auth::{check_password, check_name}, models::users::metadata::{Gender, Occupation}};

#[derive(FromForm)]
pub struct RegisterFormData<'lifetime> {
    #[field(validate = check_name())]
    display_name: &'lifetime str,
    #[field(validate = check_password(&self.confirm_password))]
    password: &'lifetime str,
    #[field(validate = check_password(&self.password))]
    confirm_password: &'lifetime str,
    gender: Gender,
    occupation: Occupation
}

