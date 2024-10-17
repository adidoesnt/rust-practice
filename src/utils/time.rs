use chrono::{DateTime, Utc};
use std::time::SystemTime;

pub fn get_timestamp_utc() -> DateTime<Utc> {
    let time: SystemTime = SystemTime::now();
    let time_utc: DateTime<Utc> = time.into();
    time_utc
}
