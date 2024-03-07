use std::sync::atomic::Ordering;

use rocket::http::Status;
use time::{Duration, OffsetDateTime};

use crate::models::{api::ApiResponse, rate_limiter::RateLimit};

impl RateLimit {
    pub fn add_to_limit_or_return(&self, message: &'static str) -> Result<(), ApiResponse> {
        if self.requests.load(Ordering::Relaxed) >= self.capacity.load(Ordering::Relaxed) {
            return Err(ApiResponse::String(Status::TooManyRequests, message));
        }

        if self
            .time_accumulator_started
            .saturating_add(Duration::seconds(30))
            < OffsetDateTime::now_utc()
        {
            self.requests.swap(0, Ordering::Relaxed);
        } else {
            self.requests.fetch_add(1, Ordering::Relaxed);
        }

        Ok(())
    }
}
