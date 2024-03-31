use rocket::FromFormField;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgHasArrayType;
use sqlx::prelude::Type;

#[derive(
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Type,
    FromFormField,
    Clone,
    Debug
)]
#[sqlx(type_name = "notificationtype", rename_all = "lowercase")]
pub enum NotificationType {
    Follows,
    CommunityEntrance,
    CommunityPosts,
}

#[derive(
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Type,
    FromFormField,
    Clone,
    Debug
)]
#[sqlx(type_name = "requeststatus", rename_all = "lowercase")]
pub enum RequestStatus {
    Pending,
    Rejected,
    Accepted,
}

#[derive(
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Type,
    FromFormField,
    Clone,
    Debug
)]
#[sqlx(type_name = "gender", rename_all = "lowercase")]
pub enum Gender {
    Male,
    Female,
    Other,
    Unknown,
}

#[derive(
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Type,
    FromFormField,
    Clone,
    Debug
)]
#[sqlx(type_name = "occupation", rename_all = "lowercase")]
pub enum Occupation {
    Student,
    Teacher,
    Engineer,
    Doctor,
    Lawyer,
    Developer,
    Artist,
    Unemployed,
    Other,
}

#[derive(
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Type,
    FromFormField,
    Clone,
    Debug
)]
#[sqlx(type_name = "userrole", rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    Moderator,
    User,
}

#[derive(
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Type,
    FromFormField,
    Clone,
    Debug
)]
#[sqlx(type_name = "accountstatus", rename_all = "lowercase")]
pub enum AccountStatus {
    Active,
    Deactivated,
    Banned,
}

#[derive(
    Hash,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Type,
    FromFormField,
    Clone,
    Debug
)]
#[sqlx(type_name = "communitycategory", rename_all = "lowercase")]
pub enum CommunityCategory {
    #[serde(rename = "art")]
    Art,
    #[serde(rename = "music")]
    Music,
    #[serde(rename = "gaming")]
    Gaming,
    #[serde(rename = "sports")]
    Sports,
    #[serde(rename = "science")]
    Science,
    #[serde(rename = "technology")]
    Technology,
    #[serde(rename = "literature")]
    Literature,
    #[serde(rename = "healthandfitness")]
    HealthAndFitness,
    #[serde(rename = "selfimprovement")]
    SelfImprovement,
    #[serde(rename = "academics")]
    Academics,
    #[serde(rename = "other")]
    Other,
}

impl PgHasArrayType for CommunityCategory {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("communitycategory[]")
    }
}

#[derive(
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Type,
    FromFormField,
    Clone,
    Debug
)]
#[sqlx(type_name = "conversationtype", rename_all = "lowercase")]
pub enum ConversationType {
    Direct,
    Group,
}
