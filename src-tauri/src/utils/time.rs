use chrono::{Utc, DateTime};

pub fn now() -> DateTime<Utc> {
    Utc::now()
}

