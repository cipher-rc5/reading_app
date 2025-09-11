// file: src/types/time_utils.rs
// description: Time and timestamp utilities

use super::errors::{AppResult, ValidationError};
use chrono::{DateTime, TimeZone, Utc};

pub fn unix_to_datetime(unix_timestamp: i64) -> AppResult<DateTime<Utc>> {
    Utc.timestamp_opt(unix_timestamp, 0)
        .single()
        .ok_or_else(|| {
            ValidationError::InvalidFormat(format!("Invalid Unix timestamp: {}", unix_timestamp))
                .into()
        })
}

pub fn datetime_to_unix(datetime: &DateTime<Utc>) -> i64 {
    datetime.timestamp()
}

pub fn current_unix_timestamp() -> i64 {
    Utc::now().timestamp()
}

pub fn relative_time_from_unix(unix_timestamp: i64) -> String {
    match unix_to_datetime(unix_timestamp) {
        Ok(datetime) => {
            let now = Utc::now();
            let duration = now.signed_duration_since(datetime);

            if duration.num_days() > 0 {
                format!("{} days ago", duration.num_days())
            } else if duration.num_hours() > 0 {
                format!("{} hours ago", duration.num_hours())
            } else if duration.num_minutes() > 0 {
                format!("{} minutes ago", duration.num_minutes())
            } else {
                "Just now".to_string()
            }
        }
        Err(_) => "Invalid date".to_string(),
    }
}
