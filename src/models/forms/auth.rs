use rocket::{form::Strict, FromForm};

use crate::{
    controllers::forms::auth::{check_name, check_password},
    models::users::metadata::Gender,
};

#[derive(FromForm)]
pub struct Password<'lifetime> {
    #[field(validate = check_password(&self.confirmation))]
    pub input: &'lifetime str,
    #[field(validate = check_password(&self.input))]
    pub confirmation: &'lifetime str,
}

#[derive(FromForm)]
pub struct RegisterFormData<'lifetime> {
    #[field(validate = check_name(), name = "username")]
    pub display_name: &'lifetime str,
    pub password: Strict<Password<'lifetime>>,
    pub gender: Gender,
}
