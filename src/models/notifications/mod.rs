use rocket::{
    serde::{Deserialize, Serialize},
    FromForm,
};
use sqlx::prelude::FromRow;
use time::OffsetDateTime;

use super::db::enums::NotificationType;

/// For realtime in-memory Rocket State notifications.
#[derive(FromForm, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct RealtimeNotification {
    pub _recipient_id: i64,
    pub _sender_id: i64,
    pub message: String,
    pub sent_at: String,
    pub link: Option<String>,
}

#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct Notification {
    pub _id: i64,
    pub _recipient_id: i64,
    pub _sender_id: i64,
    pub _created_at: OffsetDateTime,
    pub notification_type: NotificationType,
    pub is_read: Option<bool>,
    pub message: String,
    pub link: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserNotificationPreference {
    pub _user_id: i64,
    pub notification_type: NotificationType,
    pub enabled: bool,
}
