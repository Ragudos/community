use rocket::serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::models::db::enums::{CommunityCategory, UserRole};

#[derive(Clone, Debug)]
pub struct CommunityTable {
    _id: i64,
    _created_at: String,
    pub uid: String,
    pub display_name: String,
    pub categories: Vec<CommunityCategory>,
    pub description: String,
    pub owner_id: i64,
    pub is_private: bool,
    pub display_image: Option<String>,
    pub cover_image: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Community {
    pub uid: String,
    pub display_name: String,
    pub categories: Option<Vec<CommunityCategory>>,
    pub description: String,
    pub owner_id: i64,
    pub is_private: bool,
    pub display_image: Option<String>,
    pub cover_image: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CommunityWithTotalMembers {
    pub uid: String,
    pub display_name: String,
    pub categories: Option<Vec<CommunityCategory>>,
    pub description: String,
    pub owner_id: i64,
    pub is_private: bool,
    pub display_image: Option<String>,
    pub cover_image: Option<String>,
    pub total_members: Option<i64>,
}

/// Combined table for community and total of community_memberships
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CommunityHomepageCard {
    pub uid: String,
    pub display_name: String,
    pub display_image: Option<String>,
    pub cover_image: Option<String>,
    pub description: String,
    pub is_private: bool,
    pub total_members: Option<i64>,
}

/// We join tables to get all members
/// of a community, for example.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CommunityMembership {
    pub uid: String,
    pub display_name: String,
    pub display_image: Option<String>,
    pub role: UserRole,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CommunityJoinRequest {
    pub _created_at: OffsetDateTime,
    pub reason: String,
    pub user_uid: String,
}
