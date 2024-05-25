use std::time::SystemTime;

use actix_web::cookie::time::{format_description::well_known::Rfc3339, OffsetDateTime};

pub fn date_time_now() -> String {
    OffsetDateTime::from_unix_timestamp(
        SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as i64,
    )
    .expect("Invalid timestamp")
    .format(&Rfc3339)
    .expect("Failed to format time")
}
