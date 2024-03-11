use rocket::FromForm;

use crate::controllers::validate::{validate_ascii_text, validate_honeypot};

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
}
