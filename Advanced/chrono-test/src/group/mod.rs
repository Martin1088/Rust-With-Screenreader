use std::i32;

use chrono::{Datelike, NaiveDate, NaiveDateTime, TimeZone, Timelike, Utc};
pub struct TimeFunctions;
impl TimeFunctions {
    pub fn start_of_day() -> i32 {
        let today = Utc::now();
        let start_of_day = Utc
            .from_utc_date(&NaiveDate::from_ymd(
                today.year(),
                today.month(),
                today.day(),
            ))
            .and_hms(0, 0, 0);
        // Utc::today().and_hms(0, 0, 0).timestamp() as i32
        // Utc::now().timestamp() as i32
        // let test = Utc::now().hour();
        start_of_day.timestamp() as i32
    }

    pub fn end_of_day() -> i32 {
        let today = Utc::today();
        println!(" {}", today);
        let end_of_day = Utc
            .from_utc_date(&NaiveDate::from_ymd(
                today.year(),
                today.month(),
                today.day(),
            ))
            .and_hms(23, 59, 59);
        end_of_day.timestamp() as i32
    }

    pub fn timestamp_rfc3339() {
        let today = Utc::now();
        let test = today
            .date_naive()
            .and_hms_opt(23, 59, 59)
            .unwrap_or(Utc::now().date_naive().into())
            .timestamp() as i32;
        let time_now: NaiveDateTime = Utc::now().date_naive().into();
        let test_rfc = Utc::now().to_rfc3339();
        println!(" {}", today);
        println!(" {}", test);
        println!(" {}", time_now);
        println!(" {}", test_rfc);
    }
}
