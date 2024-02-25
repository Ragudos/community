use rocket::FromForm;

use crate::{
    controllers::forms::auth::{check_name, check_password},
    models::users::metadata::Gender,
};

/// Used for registration, and confirmation in sensitive actions.
#[derive(FromForm)]
pub struct Password<'lifetime> {
    #[field(validate = check_password(Some(&self.confirmation)))]
    pub input: &'lifetime str,
    #[field(validate = check_password(Some(&self.input)))]
    pub confirmation: &'lifetime str,
}

#[derive(FromForm)]
pub struct RegisterFormData<'lifetime> {
    #[field(validate = check_name(), name = "username")]
    pub display_name: &'lifetime str,
    pub password: Password<'lifetime>,
    pub gender: Gender,
}

#[derive(FromForm)]
pub struct LoginFormData<'lifetime> {
    #[field(validate = check_name(), name = "username")]
    pub display_name: &'lifetime str,
    pub password: &'lifetime str,
}

