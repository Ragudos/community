use std::sync::atomic::AtomicU32;

use time::OffsetDateTime;

#[derive(Debug)]
pub struct RateLimit {
    pub capacity: AtomicU32,
    pub time_accumulator_started: OffsetDateTime,
    pub requests: AtomicU32,
}
