use rocket::fs::TempFile;
use rocket::FromForm;

use crate::{
    controllers::{community::validate::check_name, validate::check_image},
    models::community::schema::CommunityCategory,
};

#[derive(Debug, FromForm)]
pub struct CreateCommunity<'lifetime> {
    #[field(validate = check_name(), name="community_name")]
    pub display_name: &'lifetime str,
    #[field(validate = check_image())]
    pub display_image: TempFile<'lifetime>,
    #[field(validate = check_image())]
    pub cover_image: TempFile<'lifetime>,
    #[field(validate = len(20..100))]
    pub description: &'lifetime str,
    pub is_private: bool,
    pub category: Option<CommunityCategory>,
    #[field(name = "g-recaptcha-response")]
    pub recaptcha_token: &'lifetime str,
}
