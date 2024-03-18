use std::sync::{
    atomic::{AtomicBool, AtomicU32, Ordering},
    RwLock,
};

use rocket::http::Status;
use rocket_dyn_templates::{context, Template};
use time::{Duration, OffsetDateTime};

use crate::{
    models::{Toast, ToastTypes},
    responders::ApiResponse,
};

pub struct RateLimiter {
    pub capacity: AtomicU32,
    pub time_accumulator_started: RwLock<OffsetDateTime>,
    pub did_time_accumulator_start: AtomicBool,
    pub requests: AtomicU32,
}

pub trait RateLimiterTrait {
    fn start_time_accumulator(&self) -> Result<(), ApiResponse>;
    fn add_to_limit_or_return(&self) -> Result<(), ApiResponse>;
}

impl RateLimiterTrait for RateLimiter {
    fn start_time_accumulator(&self) -> Result<(), ApiResponse> {
        let did_time_start = self.did_time_accumulator_start.load(Ordering::Acquire);

        if !did_time_start {
            let mut time_accumulator = self.time_accumulator_started.write()
            .map_err(|error| {
                eprintln!("Failed to activate Rate Limiter!: {:?}", error);

                ApiResponse::Render {
                    status: Status::InternalServerError,
                    template: Some(Template::render("partials/toast", context! {
                        toast: Toast {
                            message: "Something went wrong. Please try again later.".to_string(),
                            r#type: Some(ToastTypes::Error)
                        }
                    })),
                    headers: None
                }
            })?;

            *time_accumulator = OffsetDateTime::now_utc();
            self.did_time_accumulator_start
                .swap(true, Ordering::Release);
        }

        Ok(())
    }

    fn add_to_limit_or_return(&self) -> Result<(), ApiResponse> {
        let amount_of_requests = self.requests.load(Ordering::Acquire);
        let capacity = self.capacity.load(Ordering::Acquire);

        if amount_of_requests >= capacity {
            self.start_time_accumulator()?;

            let time_after_thirty_seconds = self.time_accumulator_started.read()
                .map_err(|error| {
                    eprintln!("Failed to activate Rate Limiter!: {:?}", error);

                    ApiResponse::Render {
                        status: Status::InternalServerError,
                        template: Some(Template::render("partials/toast", context! {
                            toast: Toast {
                                message: "Something went wrong. Please try again later.".to_string(),
                                r#type: Some(ToastTypes::Error)
                            }
                        })),
                        headers: None
                    }
                })?.saturating_add(Duration::seconds(30));
            let time_before_acceptance =
                (time_after_thirty_seconds - OffsetDateTime::now_utc()).whole_seconds();

            if time_before_acceptance.is_negative() || time_before_acceptance == 0 {
                self.requests.swap(0, Ordering::Relaxed);
                self.did_time_accumulator_start
                    .swap(false, Ordering::Release);
                return Ok(());
            }

            return Err(ApiResponse::Render {
                status: Status::TooManyRequests,
                template: Some(Template::render(
                    "partials/toast",
                    context! {
                        toast: Toast {
                            message: format!("The server is experiencing high loads of requests. Please try again in {}s.", time_before_acceptance),
                            r#type: Some(ToastTypes::Error)
                        }
                    },
                )),
                headers: None,
            });
        }

        Ok(())
    }
}
