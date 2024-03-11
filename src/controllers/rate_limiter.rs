use std::sync::atomic::Ordering;

use rocket::http::Status;
use rocket_dyn_templates::{context, Metadata};
use time::{Duration, OffsetDateTime};

use crate::models::{api::ApiResponse, rate_limiter::RateLimit, Toast, ToastTypes};

/// To be tested.
impl RateLimit {
    pub fn add_to_limit_or_return<'r>(&self, metadata: &Metadata<'r>) -> Result<(), ApiResponse> {
        if self.requests.load(Ordering::Relaxed) >= self.capacity.load(Ordering::Relaxed) {
            if !self.did_time_accumulator_start.load(Ordering::Relaxed) {
                let mut time_accumulator = self.time_accumulator_started.write().unwrap();
                *time_accumulator = OffsetDateTime::now_utc();
                self.did_time_accumulator_start
                    .swap(true, Ordering::Relaxed);
            }

            let time_after_thirty_seconds = self
                .time_accumulator_started
                .read()
                .unwrap()
                .saturating_add(Duration::seconds(30));
            let time_before_acceptance =
                (time_after_thirty_seconds - OffsetDateTime::now_utc()).whole_seconds();

            if time_before_acceptance.is_negative() || time_before_acceptance == 0 {
                self.requests.swap(0, Ordering::Relaxed);
                self.did_time_accumulator_start
                    .swap(false, Ordering::Relaxed);
                return Ok(());
            }

            let (mime, html) = metadata.render(
                "partials/components/toast",
                context! {
                    toast: Toast {
                        message: format!("The server is experiencing high loads of requests. Please try again in {}s.", time_before_acceptance),
                        r#type: Some(ToastTypes::Error)
                    }
                }
            ).unwrap();

            return Err(ApiResponse::CustomHTML(Status::TooManyRequests, mime, html));
        }

        if self
            .time_accumulator_started
            .read()
            .unwrap()
            .saturating_add(Duration::seconds(30))
            < OffsetDateTime::now_utc()
        {
            self.requests.swap(0, Ordering::Relaxed);
            self.did_time_accumulator_start
                .swap(false, Ordering::Relaxed);
        } else {
            self.requests.fetch_add(1, Ordering::Relaxed);
        }

        Ok(())
    }
}
