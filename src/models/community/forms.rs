use rocket::FromForm;

use crate::{controllers::validate::validate_honeypot, models::db::enums::CommunityCategory};

#[derive(FromForm, Debug)]
pub struct CreateCommunity<'r> {
    #[field(name = "community_name", validate = len(3..=60))]
    pub display_name: &'r str,
    #[field(validate = len(20..=255))]
    pub description: &'r str,
    #[field(validate = todo!())]
    pub categories: Vec<CommunityCategory>,
    #[field(validate = validate_honeypot())]
    pub honeypot: &'r str,
}
