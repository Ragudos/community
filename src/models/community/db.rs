use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use super::schema::Community;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommunitiesCacheResult {
    pub stored_last: OffsetDateTime,
    pub communities: Vec<Community>,
}
