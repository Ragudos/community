use rocket::serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestDeletionJWT {
    pub community_id: i64,
    pub user_id: i64,
    pub expires_in: OffsetDateTime,
}
