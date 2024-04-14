use time::{Duration, OffsetDateTime};

pub mod community;
pub mod errors;
pub mod htmx;
pub mod notifications;
pub mod rate_limiter;
pub mod users;
pub mod validate;

/// Supports up until a month.
pub fn format_time_difference(date: OffsetDateTime) -> String {
    let now = OffsetDateTime::now_utc();
    let duration = now - date;

    if duration <= Duration::seconds(60) {
        format!("{} second/s ago", duration.whole_seconds())
    } else if duration <= Duration::minutes(60) {
        format!("{} minute/s ago", duration.whole_minutes())
    } else if duration <= Duration::hours(24) {
        format!("{} hour/s ago", duration.whole_hours())
    } else if duration <= Duration::days(7) {
        format!("{} day/s ago", duration.whole_days())
    } else if duration <= Duration::weeks(4) {
        format!("{} week/s ago", duration.whole_weeks())
    } else {
        String::from("More than a month ago")
    }
}
