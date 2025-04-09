use chrono::{NaiveDate, Weekday as wd, Datelike};

pub fn middle_day(year: i32) -> Option<Weekday> {
    let is_leap_year = if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
        true
    } else {
        false
    };

    let total_days = if is_leap_year { 366 } else { 365 };

    if total_days % 2 == 0 {
        return None;
    }

    let middle_day_number = total_days / 2 + 1;

    // Calculate the date corresponding to the middle day
    let middle_day_date = NaiveDate::from_ymd_opt(year, 1, 1)
        .and_then(|start_date| start_date.checked_add_signed(chrono::Duration::days((middle_day_number - 1) as i64)))
        .expect("Failed to calculate middle day");

    Some(middle_day_date.weekday())
}