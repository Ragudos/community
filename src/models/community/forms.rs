use rocket::FromForm;

use crate::controllers::validate::{validate_ascii_text, validate_honeypot};

#[derive(FromForm, Debug)]
pub struct CreateCommunity {
    #[field(validate = len(3..=60))]
    #[field(validate = validate_ascii_text())]
    #[field(name = "community_name")]
    pub display_name: String,
    #[field(validate = len(20..=255))]
    pub description: String,
    #[field(validate = validate_honeypot())]
    pub honeypot: String,
}

#[derive(FromForm, Debug)]
pub struct JoinPublicCommunity {
    pub community_id: i64,
    #[field(validate = validate_honeypot())]
    pub honeypot: String,
}

#[derive(FromForm, Debug)]
pub struct JoinPrivateCommunity {
    #[field(validate = len(20..=255))]
    pub reason: String,
    pub community_id: i64,
    #[field(validate = validate_honeypot())]
    pub honeypot: String,
}
