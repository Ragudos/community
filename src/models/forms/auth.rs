use rocket::FromForm;

use crate::{controllers::forms::auth::{check_password, check_name}, models::users::metadata::{Gender, Occupation}};

#[derive(FromForm)]
pub struct RegisterFormData<'lifetime> {
    #[field(validate = check_name())]
    pub display_name: &'lifetime str,
    #[field(validate = check_password(&self.confirm_password))]
    pub password: &'lifetime str,
    #[field(validate = check_password(&self.password))]
    pub confirm_password: &'lifetime str,
    pub gender: Gender,
    pub occupation: Occupation
}

