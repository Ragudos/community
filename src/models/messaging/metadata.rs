use rocket::serde::{Deserialize, Serialize};
use rocket::time::OffsetDateTime;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ConversationType {
    Direct,
    Group,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConversationMetadata<'lifetime> {
    pub id: isize,
    pub conversation_type: ConversationType,
    pub created_at: &'lifetime str,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message<'lifetime> {
    pub id: isize,
    pub conversation_id: isize,
    pub user_id: isize,
    pub content: &'lifetime str,
    pub created_at: OffsetDateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserConversation {
    pub user_id: isize,
    pub conversation_id: isize,
    pub last_read_message: isize,
}
