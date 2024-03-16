use rocket::FromFormField;
use serde::Deserialize;
use serde::Serialize;
use sqlx::postgres::PgHasArrayType;
use sqlx::prelude::Type;

#[derive(Serialize, Deserialize, PartialEq, Eq, Type, FromFormField, Clone, Debug)]
#[sqlx(type_name = "gender", rename_all = "lowercase")]
pub enum Gender {
    Male,
    Female,
    Other,
    Unknown,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Type, FromFormField, Clone, Debug)]
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

#[derive(Serialize, Deserialize, PartialEq, Eq, Type, FromFormField, Clone, Debug)]
#[sqlx(type_name = "userrole", rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    Moderator,
    User,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Type, FromFormField, Clone, Debug)]
#[sqlx(type_name = "accountstatus", rename_all = "lowercase")]
pub enum AccountStatus {
    Active,
    Deactivated,
    Banned,
}

#[derive(Hash, Serialize, Deserialize, PartialEq, Eq, Type, FromFormField, Clone, Debug)]
#[sqlx(type_name = "communitycategory", rename_all = "lowercase")]
pub enum CommunityCategory {
    Art,
    Music,
    Gaming,
    Sports,
    Science,
    Technology,
    Literature,
    HealthAndFitness,
    SelfImprovement,
    Academics,
    Other,
}

impl From<&str> for CommunityCategory {
    fn from(s: &str) -> Self {
        match s {
            "art" => CommunityCategory::Art,
            "music" => CommunityCategory::Music,
            "gaming" => CommunityCategory::Gaming,
            "sports" => CommunityCategory::Sports,
            "science" => CommunityCategory::Science,
            "technology" => CommunityCategory::Technology,
            "literature" => CommunityCategory::Literature,
            "healthandfitness" => CommunityCategory::HealthAndFitness,
            "selfimprovement" => CommunityCategory::SelfImprovement,
            "academics" => CommunityCategory::Academics,
            _ => CommunityCategory::Other,
        }
    }
}

impl PgHasArrayType for CommunityCategory {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("communitycategory[]")
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Type, FromFormField, Clone, Debug)]
#[sqlx(type_name = "conversationtype", rename_all = "lowercase")]
pub enum ConversationType {
    Direct,
    Group,
}
