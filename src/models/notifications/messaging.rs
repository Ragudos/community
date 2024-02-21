use rocket::{
    serde::{Deserialize, Serialize},
    time::OffsetDateTime,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Notification {
    pub id: isize,
    pub created_at: OffsetDateTime,
    pub is_read: bool,
    pub receiver_id: isize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SystemNotificationType {
    Info,
    Warning,
    Error,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageNotification<'lifetime> {
    pub sender_id: isize,
    pub content: &'lifetime str,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SystemNotification<'lifetime> {
    pub notification_type: SystemNotificationType,
    pub content: &'lifetime str,
}
