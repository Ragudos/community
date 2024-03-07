use rocket::FromForm;

use crate::{controllers::validate::validate_honeypot, models::db::enums::CommunityCategory};

#[derive(FromForm)]
pub struct CreateCommunity<'r> {
    pub display_name: &'r str,
    pub description: &'r str,
    pub categories: Vec<CommunityCategory>,
    #[field(validate = validate_honeypot())]
    pub honeypot: &'r str,
}
