use chrono::{Datelike, NaiveDateTime, TimeZone, Utc};

pub fn now_timestamp_s() -> i64 {
    Utc::now().timestamp()
}

pub fn now_timestamp_ms() -> i64 {
    Utc::now().timestamp_millis()
}

pub fn get_timestamp_s(time: &str) -> i64 {
    let time = NaiveDateTime::parse_from_str(
        time, "%Y-%m-%d %H:%M:%S").unwrap();
    let time = Utc.from_utc_datetime(&time);
    time.timestamp()
}

pub fn get_month_timestamp_s() -> i64 {
    let year = Utc::now().year();
    let month = Utc::now().month();
    let first_day = Utc.with_ymd_and_hms(year, month, 1, 0, 0, 0);
    first_day.unwrap().timestamp()
}
