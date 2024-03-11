use std::sync::{
    atomic::{AtomicBool, AtomicU32},
    RwLock,
};

use time::OffsetDateTime;

#[derive(Debug)]
pub struct RateLimit {
    pub capacity: AtomicU32,
    pub time_accumulator_started: RwLock<OffsetDateTime>,
    pub did_time_accumulator_start: AtomicBool,
    pub requests: AtomicU32,
}
