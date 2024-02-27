use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::models::users::metadata::UserRole;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Community {
    pub id: i32,
    pub display_name: String,
    pub display_image: String,
    pub cover_image: Option<String>,
    pub description: String,
    pub owner_id: i32,
    pub is_private: bool,
    pub category: Option<String>,
    pub created_at: OffsetDateTime,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CommunityMembership {
    pub user_id: i32,
    pub community_id: i32,
    pub joined_at: OffsetDateTime,
    pub role: UserRole,
}
