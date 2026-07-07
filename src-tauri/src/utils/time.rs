use chrono::{SecondsFormat, Utc, DateTime};

pub fn now() -> DateTime<Utc> {
    Utc::now()
}

