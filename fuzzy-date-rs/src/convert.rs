use chrono::{DateTime, Datelike, Duration, FixedOffset, Month, NaiveDate, Timelike};
use std::cmp;

#[derive(PartialEq)]
pub(crate) enum Change {
    First,
    Last,
    Prev,
    Next,
    None,
}

/// Move datetime into specified year, month and day from a basic ISO8601 value
pub(crate) fn date_iso8601(from_time: DateTime<FixedOffset>, value: String) -> Result<DateTime<FixedOffset>, ()> {
    if value.len().ne(&8) {
        return Err(());
    }

    let Ok(year) = value[0..4].parse::<i64>() else {
        return Err(());
    };

    let Ok(month) = value[4..6].parse::<i64>() else {
        return Err(());
    };

    let Ok(day) = value[6..8].parse::<i64>() else {
        return Err(());
    };

    date_ymd(from_time, year, month, day)
}

/// Get timestamp for specified timestamp, always in UTC
pub(crate) fn date_stamp(sec: i64, ms: i64) -> DateTime<FixedOffset> {
    let nano_sec = (ms * 1_000_000) as u32;
    DateTime::from_timestamp(sec, nano_sec).unwrap().fixed_offset()
}

/// Move datetime into specified year, month and day
pub(crate) fn date_ymd(
    from_time: DateTime<FixedOffset>,
    year: i64,
    month: i64,
    day: i64,
) -> Result<DateTime<FixedOffset>, ()> {
    let new_time = from_time.with_day(1).unwrap();

    let new_time = match new_time.with_year(year as i32) {
        Some(v) => v,
        None => return Err(()),
    };

    let new_time = match new_time.with_month(month as u32) {
        Some(v) => v,
        None => return Err(()),
    };

    let new_time = match new_time.with_day(day as u32) {
        Some(v) => v,
        None => return Err(()),
    };

    Ok(new_time)
}

/// Move datetime into specified year and week
pub(crate) fn date_yw(
    from_time: DateTime<FixedOffset>,
    year: i64,
    week: i64,
    start_day: i8,
) -> Result<DateTime<FixedOffset>, ()> {
    if week.lt(&1) || week.gt(&53) {
        return Err(());
    }

    let iso_week = match NaiveDate::from_isoywd_opt(year as i32, week as u32, chrono::Weekday::Mon) {
        Some(v) => v,
        None => return Err(()),
    };

    let new_time = date_ymd(from_time, iso_week.year() as i64, iso_week.month() as i64, iso_week.day() as i64)?;

    match start_day {
        1 => Ok(new_time),
        _ => Ok(offset_weekday(new_time, start_day as i64, Change::Prev)),
    }
}

/// Return time set to the last day of given year and month
pub(crate) fn into_last_of_month(
    from_time: DateTime<FixedOffset>,
    year: i64,
    month: i64,
) -> Result<DateTime<FixedOffset>, ()> {
    let last_day = into_month_day(year as i32, month as u32, 31) as i64;
    date_ymd(from_time, year, month, last_day)
}

/// Return either the day given if given month has enough days
/// to use it, or the last day of the month
pub(crate) fn into_month_day(year: i32, month: u32, day: u32) -> u32 {
    if day.le(&28) {
        return day;
    }

    let Ok(target_month) = Month::try_from(month as u8) else {
        return day;
    };

    match target_month.num_days(year) {
        Some(v) => cmp::min(v as u32, day),
        None => day,
    }
}

/// Move datetime into previous or upcoming month
pub(crate) fn offset_month(from_time: DateTime<FixedOffset>, new_month: i64, change: Change) -> DateTime<FixedOffset> {
    let curr_month: i64 = from_time.month() as i64;

    if change.eq(&Change::None) && curr_month.eq(&new_month) {
        return from_time;
    }

    let move_by: i64 = match change {
        Change::Prev => match new_month.lt(&curr_month) {
            true => 0 - (curr_month - new_month),
            false => -12 - (curr_month - new_month),
        },
        _ => match new_month.le(&curr_month) {
            true => 12 - (curr_month - new_month),
            false => new_month - curr_month,
        },
    };

    offset_months(from_time, move_by)
}

/// Move datetime by given amount of months
pub(crate) fn offset_months(from_time: DateTime<FixedOffset>, amount: i64) -> DateTime<FixedOffset> {
    let new_month: i32 = from_time.month() as i32 + amount as i32;

    if new_month.ge(&1) && new_month.le(&12) {
        let target_day: u32 = into_month_day(from_time.year(), new_month as u32, from_time.day());

        return from_time.with_day(target_day).unwrap().with_month(new_month as u32).unwrap();
    }

    let offset_months: u32 = (new_month as f64).abs() as u32;
    let offset_years: i8 = ((offset_months / 12) as f64).floor() as i8;

    let target_month: u32 = match new_month.lt(&1) {
        true => 12 - (offset_months - (offset_years as u32) * 12),
        false => from_time.month() + amount as u32 - (12 * offset_years as u32),
    };

    let target_year: i32 = match new_month.lt(&1) {
        true => from_time.year() - (offset_years as i32) - 1,
        false => from_time.year() + offset_years as i32,
    };

    let target_day: u32 = into_month_day(target_year, target_month, from_time.day());

    from_time
        .with_day(target_day)
        .unwrap()
        .with_month(target_month)
        .unwrap()
        .with_year(target_year)
        .unwrap()
}

/// Move datetime into first or last of the specified year and month
pub(crate) fn offset_range_year_month(
    from_time: DateTime<FixedOffset>,
    year: i64,
    month: i64,
    change: Change,
) -> Result<DateTime<FixedOffset>, ()> {
    if change.eq(&Change::First) {
        return date_ymd(from_time, year, month, 1);
    }

    if change.eq(&Change::Last) {
        return into_last_of_month(from_time, year, month);
    }

    Ok(from_time)
}

/// Move datetime into first or last weekday of specified year and month
pub(crate) fn offset_range_year_month_wday(
    from_time: DateTime<FixedOffset>,
    year: i64,
    month: i64,
    wday: i64,
    change: Change,
) -> Result<DateTime<FixedOffset>, ()> {
    if change.eq(&Change::First) {
        let from_time = date_ymd(from_time, year, month, 1)?;
        let first_wday = from_time.weekday().num_days_from_monday() as i64 + 1;
        let move_days = match wday.lt(&first_wday) {
            true => Duration::weeks(1) + Duration::days(wday - first_wday),
            false => Duration::days(wday - first_wday),
        };
        return Ok(from_time + move_days);
    }

    if change.eq(&Change::Last) {
        let from_time = into_last_of_month(from_time, year, month)?;
        let last_wday = from_time.weekday().num_days_from_monday() as i64 + 1;
        let move_days = match wday.gt(&last_wday) {
            true => Duration::weeks(1) + Duration::days(last_wday - wday),
            false => Duration::days(last_wday - wday),
        };
        return Ok(from_time - move_days);
    }

    Ok(from_time)
}

/// Move datetime into previous or upcoming weekday
pub(crate) fn offset_weekday(
    from_time: DateTime<FixedOffset>,
    new_weekday: i64,
    change: Change,
) -> DateTime<FixedOffset> {
    let curr_weekday: i64 = from_time.weekday().num_days_from_monday() as i64 + 1;

    let mut offset_weeks: i64 = 0;

    if change.eq(&Change::Prev) && curr_weekday.le(&new_weekday) {
        offset_weeks = -1;
    } else if change.eq(&Change::Next) && curr_weekday.ge(&new_weekday) {
        offset_weeks = 1;
    }

    from_time + Duration::weeks(offset_weeks) + Duration::days(new_weekday - curr_weekday)
}

/// Move datetime by given amount of weeks, to the start of the week
pub(crate) fn offset_weeks(from_time: DateTime<FixedOffset>, amount: i64, start_day: i8) -> DateTime<FixedOffset> {
    let days_since_start: i64 = match start_day {
        1 => from_time.weekday().num_days_from_monday() as i64,
        _ => from_time.weekday().num_days_from_sunday() as i64,
    };

    from_time - Duration::days(days_since_start) + Duration::weeks(amount)
}

/// Move datetime by given amount of years
pub(crate) fn offset_years(from_time: DateTime<FixedOffset>, amount: i64) -> DateTime<FixedOffset> {
    let new_year: i32 = from_time.year() + amount as i32;

    if from_time.month() != 2 {
        return from_time.with_year(new_year).unwrap();
    }

    from_time
        .with_day(1)
        .unwrap()
        .with_year(new_year)
        .unwrap()
        .with_day(into_month_day(new_year, 2, from_time.day()))
        .unwrap()
}

// Move datetime into specified 12-hour, minute and second
pub(crate) fn time_12h(
    from_time: DateTime<FixedOffset>,
    hour: i64,
    min: i64,
    sec: i64,
    meridiem: i64,
) -> Result<DateTime<FixedOffset>, ()> {
    if hour.lt(&1) || hour.gt(&12) {
        return Err(());
    }

    let hour = match hour.eq(&12) {
        true => {
            if meridiem.eq(&1) {
                0
            } else {
                12
            }
        }
        false => {
            if meridiem.eq(&1) {
                hour
            } else {
                hour + 12
            }
        }
    };

    time_hms(from_time, hour, min, sec, 0)
}

// Move datetime into specified hour, minute and second
pub(crate) fn time_hms(
    from_time: DateTime<FixedOffset>,
    hour: i64,
    min: i64,
    sec: i64,
    ms: i64,
) -> Result<DateTime<FixedOffset>, ()> {
    if hour.lt(&0) || min.lt(&0) || sec.lt(&0) || ms.lt(&0) {
        return Err(());
    }

    if hour.gt(&23) || min.gt(&59) || sec.gt(&59) || ms.gt(&999) {
        return Err(());
    }

    Ok(from_time
        .with_hour(hour as u32)
        .unwrap()
        .with_minute(min as u32)
        .unwrap()
        .with_second(sec as u32)
        .unwrap()
        .with_nanosecond((ms * 1_000_000) as u32)
        .unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_iso8601_success() {
        let from_time = into_datetime("2022-01-31T15:22:28+02:00");

        let expect = vec![
            ("00900101", "0090-01-01 15:22:28 +02:00"),
            ("10000101", "1000-01-01 15:22:28 +02:00"),
            ("20220225", "2022-02-25 15:22:28 +02:00"),
            ("20240229", "2024-02-29 15:22:28 +02:00"),
        ];

        for (from_value, expect_value) in expect {
            assert_eq!(date_iso8601(from_time, from_value.to_string()).unwrap().to_string(), expect_value);
        }
    }

    #[test]
    fn test_date_iso8601_failure() {
        let from_time = into_datetime("2022-01-31T15:22:28+02:00");

        let expect = vec![
            "2022025",  // Value is too short
            "00000025", // Year is 0
            "20220025", // Month is 0
            "20220200", // Day is 0
            "20241310", // Invalid month
            "20240230", // Non-existent date
        ];

        for from_value in expect {
            assert!(date_iso8601(from_time, from_value.to_string()).is_err());
        }
    }

    #[test]
    fn test_date_stamp() {
        assert_eq!(date_stamp(0, 0).to_string(), "1970-01-01 00:00:00 +00:00");
        assert_eq!(date_stamp(-100, 0).to_string(), "1969-12-31 23:58:20 +00:00");
        assert_eq!(date_stamp(1705072948, 0).to_string(), "2024-01-12 15:22:28 +00:00");
        assert_eq!(date_stamp(1705072948, 544).to_string(), "2024-01-12 15:22:28.544 +00:00");
    }

    #[test]
    fn test_date_ymd() {
        let from_time = into_datetime("2022-01-31T15:22:28+02:00");

        assert_eq!(date_ymd(from_time, 2022, 2, 25).unwrap().to_string(), "2022-02-25 15:22:28 +02:00",);
        assert_eq!(date_ymd(from_time, 2024, 2, 29).unwrap().to_string(), "2024-02-29 15:22:28 +02:00",);

        assert!(date_ymd(from_time, 2024, 13, 10).is_err());
        assert!(date_ymd(from_time, 2024, 2, 30).is_err());
    }

    #[test]
    fn test_date_yw() {
        let from_time = into_datetime("2022-01-31T15:22:28+02:00");

        assert_eq!(date_yw(from_time, 2020, 53, 1).unwrap().to_string(), "2020-12-28 15:22:28 +02:00",);
        assert_eq!(date_yw(from_time, 2020, 53, 7).unwrap().to_string(), "2020-12-27 15:22:28 +02:00",);
        assert_eq!(date_yw(from_time, 2025, 1, 1).unwrap().to_string(), "2024-12-30 15:22:28 +02:00",);
        assert_eq!(date_yw(from_time, 2025, 1, 7).unwrap().to_string(), "2024-12-29 15:22:28 +02:00",);

        assert!(date_yw(from_time, 2020, 0, 1).is_err());
        assert!(date_yw(from_time, 2020, 54, 1).is_err());
    }

    #[test]
    fn test_into_last_of_month() {
        let expect: Vec<(&str, i64, i64, &str)> = vec![
            ("2024-01-01T15:02:11+02:00", 2024, 2, "2024-02-29 15:02:11 +02:00"),
            ("2024-01-01T15:02:11+02:00", 2025, 12, "2025-12-31 15:02:11 +02:00"),
        ];

        for (from_time, new_year, new_month, expect_time) in expect {
            let result = into_last_of_month(into_datetime(from_time), new_year, new_month);
            assert_eq!(result.unwrap().to_string(), expect_time);
        }
    }

    #[test]
    fn test_into_month_day() {
        assert_eq!(into_month_day(2024, 2, 1), 1);
        assert_eq!(into_month_day(2024, 2, 29), 29);
        assert_eq!(into_month_day(2024, 2, 30), 29);
    }

    #[test]
    fn test_offset_month() {
        let expect: Vec<(&str, i64, Change, &str)> = vec![
            // This
            ("2022-02-23T15:22:28+02:00", 1, Change::None, "2023-01-23 15:22:28 +02:00"),
            ("2022-02-23T15:22:28+02:00", 2, Change::None, "2022-02-23 15:22:28 +02:00"),
            ("2022-02-23T15:22:28+02:00", 3, Change::None, "2022-03-23 15:22:28 +02:00"),
            ("2022-02-23T15:22:28+02:00", 4, Change::None, "2022-04-23 15:22:28 +02:00"),
            ("2022-02-23T15:22:28+02:00", 5, Change::None, "2022-05-23 15:22:28 +02:00"),
            ("2022-02-23T15:22:28+02:00", 6, Change::None, "2022-06-23 15:22:28 +02:00"),
            ("2022-02-23T15:22:28+02:00", 7, Change::None, "2022-07-23 15:22:28 +02:00"),
            ("2022-02-23T15:22:28+02:00", 8, Change::None, "2022-08-23 15:22:28 +02:00"),
            ("2022-02-23T15:22:28+02:00", 9, Change::None, "2022-09-23 15:22:28 +02:00"),
            ("2022-02-23T15:22:28+02:00", 10, Change::None, "2022-10-23 15:22:28 +02:00"),
            ("2022-02-23T15:22:28+02:00", 11, Change::None, "2022-11-23 15:22:28 +02:00"),
            ("2022-02-23T15:22:28+02:00", 12, Change::None, "2022-12-23 15:22:28 +02:00"),
            // Prev
            ("2022-06-23T15:22:28+02:00", 1, Change::Prev, "2022-01-23 15:22:28 +02:00"),
            ("2022-06-23T15:22:28+02:00", 2, Change::Prev, "2022-02-23 15:22:28 +02:00"),
            ("2022-06-23T15:22:28+02:00", 3, Change::Prev, "2022-03-23 15:22:28 +02:00"),
            ("2022-06-23T15:22:28+02:00", 4, Change::Prev, "2022-04-23 15:22:28 +02:00"),
            ("2022-06-23T15:22:28+02:00", 5, Change::Prev, "2022-05-23 15:22:28 +02:00"),
            ("2022-06-23T15:22:28+02:00", 6, Change::Prev, "2021-06-23 15:22:28 +02:00"),
            ("2022-06-23T15:22:28+02:00", 7, Change::Prev, "2021-07-23 15:22:28 +02:00"),
            ("2022-06-23T15:22:28+02:00", 8, Change::Prev, "2021-08-23 15:22:28 +02:00"),
            ("2022-06-23T15:22:28+02:00", 9, Change::Prev, "2021-09-23 15:22:28 +02:00"),
            ("2022-06-23T15:22:28+02:00", 10, Change::Prev, "2021-10-23 15:22:28 +02:00"),
            ("2022-06-23T15:22:28+02:00", 11, Change::Prev, "2021-11-23 15:22:28 +02:00"),
            ("2022-06-23T15:22:28+02:00", 12, Change::Prev, "2021-12-23 15:22:28 +02:00"),
            // Next
            ("2022-06-23T15:22:28+02:00", 1, Change::Next, "2023-01-23 15:22:28 +02:00"),
            ("2022-06-23T15:22:28+02:00", 2, Change::Next, "2023-02-23 15:22:28 +02:00"),
            ("2022-06-23T15:22:28+02:00", 3, Change::Next, "2023-03-23 15:22:28 +02:00"),
            ("2022-06-23T15:22:28+02:00", 4, Change::Next, "2023-04-23 15:22:28 +02:00"),
            ("2022-06-23T15:22:28+02:00", 5, Change::Next, "2023-05-23 15:22:28 +02:00"),
            ("2022-06-23T15:22:28+02:00", 6, Change::Next, "2023-06-23 15:22:28 +02:00"),
            ("2022-06-23T15:22:28+02:00", 7, Change::Next, "2022-07-23 15:22:28 +02:00"),
            ("2022-06-23T15:22:28+02:00", 8, Change::Next, "2022-08-23 15:22:28 +02:00"),
            ("2022-06-23T15:22:28+02:00", 9, Change::Next, "2022-09-23 15:22:28 +02:00"),
            ("2022-06-23T15:22:28+02:00", 10, Change::Next, "2022-10-23 15:22:28 +02:00"),
            ("2022-06-23T15:22:28+02:00", 11, Change::Next, "2022-11-23 15:22:28 +02:00"),
            ("2022-06-23T15:22:28+02:00", 12, Change::Next, "2022-12-23 15:22:28 +02:00"),
            // Day is automatically adjusted
            ("2022-01-30T15:22:28+02:00", 2, Change::Next, "2022-02-28 15:22:28 +02:00"),
        ];

        for (from_time, new_month, change, expect_time) in expect {
            let result_time = offset_month(into_datetime(from_time), new_month, change);
            assert_eq!(result_time.to_string(), expect_time);
        }
    }

    #[test]
    fn test_offset_months() {
        let expect: Vec<(&str, i64, &str)> = vec![
            ("2024-01-31T15:22:28+02:00", 0, "2024-01-31 15:22:28 +02:00"),
            ("2024-01-31T15:22:28+02:00", -1, "2023-12-31 15:22:28 +02:00"),
            ("2024-01-31T15:22:28+02:00", -24, "2022-01-31 15:22:28 +02:00"),
            ("2024-01-31T15:22:28+02:00", 1, "2024-02-29 15:22:28 +02:00"),
            ("2024-01-31T15:22:28+02:00", 24, "2026-01-31 15:22:28 +02:00"),
        ];

        for (from_time, move_months, expect_time) in expect {
            let result_time = offset_months(into_datetime(from_time), move_months);
            assert_eq!(result_time.to_string(), expect_time);
        }
    }

    #[test]
    fn test_offset_range_year_months() {
        let expect: Vec<(&str, i64, i64, Change, &str)> = vec![
            ("2024-01-31T15:22:28+02:00", 2024, 2, Change::None, "2024-01-31 15:22:28 +02:00"),
            ("2024-01-31T15:22:28+02:00", 2024, 2, Change::First, "2024-02-01 15:22:28 +02:00"),
            ("2024-01-31T15:22:28+02:00", 2024, 2, Change::Last, "2024-02-29 15:22:28 +02:00"),
            // Change year
            ("2024-01-31T15:22:28+02:00", 2025, 2, Change::None, "2024-01-31 15:22:28 +02:00"),
            ("2024-01-31T15:22:28+02:00", 2025, 2, Change::First, "2025-02-01 15:22:28 +02:00"),
            ("2024-01-31T15:22:28+02:00", 2025, 2, Change::Last, "2025-02-28 15:22:28 +02:00"),
        ];

        for (from_time, new_year, new_month, change, expect_time) in expect {
            let result_time = offset_range_year_month(into_datetime(from_time), new_year, new_month, change);
            assert_eq!(result_time.unwrap().to_string(), expect_time);
        }
    }

    #[test]
    fn test_offset_range_year_month_wdays() {
        let monday = 1;

        let expect: Vec<(&str, i64, i64, i64, Change, &str)> = vec![
            ("2024-01-31T15:22:28+02:00", 2024, 2, monday, Change::None, "2024-01-31 15:22:28 +02:00"),
            ("2024-01-31T15:22:28+02:00", 2024, 2, monday, Change::First, "2024-02-05 15:22:28 +02:00"),
            ("2024-01-31T15:22:28+02:00", 2024, 2, monday, Change::Last, "2024-02-26 15:22:28 +02:00"),
        ];

        for (from_time, new_year, new_month, new_wday, change, expect_time) in expect {
            let from_time = into_datetime(from_time);
            let result_time = offset_range_year_month_wday(from_time, new_year, new_month, new_wday, change);
            assert_eq!(result_time.unwrap().to_string(), expect_time);
        }
    }

    #[test]
    fn test_offset_weekdays() {
        let expect: Vec<(&str, i64, Change, &str)> = vec![
            ("2022-02-23T15:22:28+02:00", 1, Change::None, "2022-02-21 15:22:28 +02:00"),
            ("2022-02-23T15:22:28+02:00", 2, Change::None, "2022-02-22 15:22:28 +02:00"),
            ("2022-02-23T15:22:28+02:00", 3, Change::None, "2022-02-23 15:22:28 +02:00"),
            ("2022-02-23T15:22:28+02:00", 4, Change::None, "2022-02-24 15:22:28 +02:00"),
            ("2022-02-23T15:22:28+02:00", 5, Change::None, "2022-02-25 15:22:28 +02:00"),
            ("2022-02-23T15:22:28+02:00", 6, Change::None, "2022-02-26 15:22:28 +02:00"),
            ("2022-02-23T15:22:28+02:00", 7, Change::None, "2022-02-27 15:22:28 +02:00"),
            ("2022-02-23T15:22:28+02:00", 1, Change::Prev, "2022-02-21 15:22:28 +02:00"),
            ("2022-02-23T15:22:28+02:00", 2, Change::Prev, "2022-02-22 15:22:28 +02:00"),
            ("2022-02-23T15:22:28+02:00", 3, Change::Prev, "2022-02-16 15:22:28 +02:00"),
            ("2022-02-23T15:22:28+02:00", 4, Change::Prev, "2022-02-17 15:22:28 +02:00"),
            ("2022-02-23T15:22:28+02:00", 5, Change::Prev, "2022-02-18 15:22:28 +02:00"),
            ("2022-02-23T15:22:28+02:00", 6, Change::Prev, "2022-02-19 15:22:28 +02:00"),
            ("2022-02-23T15:22:28+02:00", 7, Change::Prev, "2022-02-20 15:22:28 +02:00"),
            ("2022-02-23T15:22:28+02:00", 1, Change::Next, "2022-02-28 15:22:28 +02:00"),
            ("2022-02-23T15:22:28+02:00", 2, Change::Next, "2022-03-01 15:22:28 +02:00"),
            ("2022-02-23T15:22:28+02:00", 3, Change::Next, "2022-03-02 15:22:28 +02:00"),
            ("2022-02-23T15:22:28+02:00", 4, Change::Next, "2022-02-24 15:22:28 +02:00"),
            ("2022-02-23T15:22:28+02:00", 5, Change::Next, "2022-02-25 15:22:28 +02:00"),
            ("2022-02-23T15:22:28+02:00", 6, Change::Next, "2022-02-26 15:22:28 +02:00"),
            ("2022-02-23T15:22:28+02:00", 7, Change::Next, "2022-02-27 15:22:28 +02:00"),
        ];

        for (from_time, new_weekday, change, expect_time) in expect {
            let result_time = offset_weekday(into_datetime(from_time), new_weekday, change);
            assert_eq!(result_time.to_string(), expect_time);
        }
    }

    #[test]
    fn test_offset_weeks() {
        let expect: Vec<(&str, i64, i8, &str)> = vec![
            // Monday as start of week
            ("2022-02-28T15:22:28+02:00", 0, 1, "2022-02-28 15:22:28 +02:00"),
            ("2023-03-21T12:00:00+02:00", -1, 1, "2023-03-13 12:00:00 +02:00"),
            ("2023-03-21T12:00:00+02:00", -25, 1, "2022-09-26 12:00:00 +02:00"),
            ("2023-03-21T12:00:00+02:00", 1, 1, "2023-03-27 12:00:00 +02:00"),
            ("2023-03-21T12:00:00+02:00", 125, 1, "2025-08-11 12:00:00 +02:00"),
            // Sunday as start of week
            ("2022-02-28T15:22:28+02:00", 0, 7, "2022-02-27 15:22:28 +02:00"),
            ("2023-03-21T12:00:00+02:00", -1, 7, "2023-03-12 12:00:00 +02:00"),
            ("2023-03-21T12:00:00+02:00", -25, 7, "2022-09-25 12:00:00 +02:00"),
            ("2023-03-21T12:00:00+02:00", 1, 7, "2023-03-26 12:00:00 +02:00"),
            ("2023-03-21T12:00:00+02:00", 125, 7, "2025-08-10 12:00:00 +02:00"),
        ];

        for (from_time, move_weeks, start_weekday, expect_time) in expect {
            let result_time = offset_weeks(into_datetime(from_time), move_weeks, start_weekday);
            assert_eq!(result_time.to_string(), expect_time);
        }
    }

    #[test]
    fn test_offset_years() {
        let expect: Vec<(&str, i64, &str)> = vec![
            ("2022-02-28T15:22:28+02:00", 0, "2022-02-28 15:22:28 +02:00"),
            ("2022-03-31T15:22:28+02:00", 1, "2023-03-31 15:22:28 +02:00"),
            // From leap year to non-leap year
            ("2024-02-29T15:22:28+02:00", -1, "2023-02-28 15:22:28 +02:00"),
        ];

        for (from_time, move_years, expect_time) in expect {
            let result_time = offset_years(into_datetime(from_time), move_years);
            assert_eq!(result_time.to_string(), expect_time);
        }
    }

    #[test]
    fn test_time_hms() {
        let from_time = into_datetime("2022-02-28T15:22:28+02:00");

        assert_eq!(time_hms(from_time, 0, 0, 0, 0).unwrap().to_string(), "2022-02-28 00:00:00 +02:00",);

        assert_eq!(time_hms(from_time, 23, 15, 1, 0).unwrap().to_string(), "2022-02-28 23:15:01 +02:00",);

        assert!(time_hms(from_time, -1, 0, 0, 0).is_err());
        assert!(time_hms(from_time, 24, 0, 0, 0).is_err());

        assert!(time_hms(from_time, 0, -1, 0, 0).is_err());
        assert!(time_hms(from_time, 0, 60, 0, 0).is_err());

        assert!(time_hms(from_time, 0, 0, -1, 0).is_err());
        assert!(time_hms(from_time, 0, 0, 60, 0).is_err());
    }

    #[test]
    fn test_time_12h() {
        let from_time = into_datetime("2022-02-28T15:22:28+02:00");

        for hour in 1..=11 {
            assert_eq!(time_12h(from_time, hour, 0, 0, 1).unwrap().hour() as i64, hour);
            assert_eq!(time_12h(from_time, hour, 0, 0, 2).unwrap().hour() as i64, hour + 12);
        }

        // 12 AM, PM
        assert_eq!(time_12h(from_time, 12, 0, 0, 1).unwrap().hour() as i64, 0);
        assert_eq!(time_12h(from_time, 12, 0, 0, 2).unwrap().hour() as i64, 12);

        assert!(time_12h(from_time, 0, 0, 0, 1).is_err());
        assert!(time_12h(from_time, 13, 0, 0, 1).is_err());
    }

    fn into_datetime(time_str: &str) -> DateTime<FixedOffset> {
        DateTime::parse_from_rfc3339(time_str).unwrap()
    }
}
