use rocket::FromForm;
use serde::Serialize;

use super::db::enums::CommunityCategory;

#[derive(FromForm, Serialize, Clone, Debug)]
pub struct ListQuery<'r> {
    pub search: Option<&'r str>,
    pub category: Option<CommunityCategory>,
    pub offset: Option<i64>,
}
