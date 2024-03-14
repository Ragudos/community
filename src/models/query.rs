use rocket::FromForm;
use serde::Serialize;

use super::db::enums::CommunityCategory;

#[derive(FromForm, Serialize)]
pub struct ListQuery<'r> {
    pub search: Option<&'r str>,
    pub category: Option<Vec<CommunityCategory>>,
    pub offset: Option<i64>,
}
