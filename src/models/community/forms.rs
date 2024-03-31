use rocket::FromForm;

use crate::controllers::validate::{
    validate_ascii_text, validate_honeypot, validate_password,
    validate_positive_integer,
};

#[derive(FromForm, Debug)]
pub struct CreateCommunity<'r> {
    #[field(validate = len(3..=60))]
    #[field(validate = validate_ascii_text())]
    #[field(name = "community_name")]
    pub display_name: &'r str,
    #[field(validate = len(20..=255))]
    pub description: &'r str,
    #[field(validate = validate_honeypot())]
    pub honeypot: &'r str,
    pub authenticity_token: &'r str,
}

#[derive(FromForm, Debug)]
pub struct JoinPublicCommunity<'r> {
    pub community_id: i64,
    #[field(validate = validate_honeypot())]
    pub honeypot: &'r str,
    pub authenticity_token: &'r str,
}

#[derive(FromForm, Debug)]
pub struct JoinPrivateCommunity<'r> {
    #[field(validate = len(20..=255))]
    pub reason: &'r str,
    pub community_id: i64,
    #[field(validate = validate_honeypot())]
    pub honeypot: &'r str,
    pub authenticity_token: &'r str,
}

#[derive(FromForm, Debug)]
pub struct EditDisplayName<'r> {
    #[field(validate = len(3..=60))]
    #[field(validate = validate_ascii_text())]
    #[field(name = "community_name")]
    pub display_name: &'r str,
    #[field(validate = validate_honeypot())]
    pub honeypot: &'r str,
    pub authenticity_token: &'r str,
    #[field(validate = validate_positive_integer())]
    pub community_id: i64,
}

#[derive(FromForm, Debug)]
pub struct RequestDeletion<'r> {
    #[field(validate = validate_positive_integer())]
    pub community_id: i64,
    pub authenticity_token: &'r str,
}

#[derive(FromForm, Debug)]
pub struct LeaveCommunity<'r> {
    #[field(validate = validate_positive_integer())]
    pub community_id: i64,
    pub authenticity_token: &'r str,
}

#[derive(FromForm, Debug)]
pub struct RequestChangeJoinProcess<'r> {
    #[field(validate = validate_positive_integer())]
    pub community_id: i64,
    pub authenticity_token: &'r str,
}

#[derive(FromForm, Debug)]
pub struct DeleteCommunity<'r> {
    #[field(validate = len(8..=64))]
    #[field(validate = validate_password())]
    pub user_password: &'r str,
    pub authenticity_token: &'r str,
    #[field(validate = validate_honeypot())]
    pub honeypot: &'r str,
}

#[derive(FromForm, Debug)]
pub struct ChangeJoinProcessCommunity<'r> {
    #[field(validate = len(8..=64))]
    #[field(validate = validate_password())]
    pub user_password: &'r str,
    pub authenticity_token: &'r str,
    #[field(validate = validate_honeypot())]
    pub honeypot: &'r str,
}
