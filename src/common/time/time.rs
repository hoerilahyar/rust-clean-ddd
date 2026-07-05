use chrono::{NaiveDateTime, Utc};
use chrono_tz::Asia::Jakarta;

pub fn now_jakarta() -> NaiveDateTime {
    Utc::now().with_timezone(&Jakarta).naive_local()
}
