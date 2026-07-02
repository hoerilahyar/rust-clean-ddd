use chrono::{DateTime, Local, Utc};

pub fn now() -> DateTime<Utc> {
    Utc::now()
}

pub fn local() -> DateTime<Local> {
    Local::now()
}

pub fn timestamp() -> i64 {
    Utc::now().timestamp()
}
