use chrono::{DateTime, Datelike, Duration, FixedOffset, NaiveDate, Timelike};
use std::cmp;

#[derive(PartialEq)]
pub(crate) enum Change {
    First,
    Last,
    Prev,
    Next,
    None,
}

/// Get timestamp for specified timestamp, always in UTC
pub(crate) fn date_stamp(sec: i64, ms: i64) -> DateTime<FixedOffset> {
    let nano_sec = (ms * 1_000_000) as u32;
    DateTime::from_timestamp(sec, nano_sec).unwrap().fixed_offset()
}

/// Move datetime into specified year, month and day
pub(crate) fn date_ymd(from_time: DateTime<FixedOffset>, year: i64, month: i64, day: i64) -> Result<DateTime<FixedOffset>, ()> {
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

/// Return either the day given if given month has enough days
/// to use it, or the last day of the month
pub(crate) fn into_month_day(year: i32, month: u32, day: u32) -> u32 {
    if day.le(&28) {
        return day;
    }

    let next_month: u32 = match month {
        12 => 1,
        _ => month + 1,
    };

    let next_year: i32 = match month {
        12 => year + 1,
        _ => year,
    };

    let month_start = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let month_end = NaiveDate::from_ymd_opt(next_year, next_month, 1).unwrap();
    let max_days: u32 = month_end.signed_duration_since(month_start).num_days() as u32;

    cmp::min(max_days, day)
}

/// Move datetime by given amount of months
pub(crate) fn offset_months(from_time: DateTime<FixedOffset>, amount: i64) -> DateTime<FixedOffset> {
    let new_month: i32 = from_time.month() as i32 + amount as i32;

    if new_month.ge(&1) && new_month.le(&12) {
        let target_day: u32 = into_month_day(
            from_time.year(), new_month as u32, from_time.day(),
        );

        return from_time
            .with_day(target_day).unwrap()
            .with_month(new_month as u32).unwrap();
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

    let target_day: u32 = into_month_day(
        target_year, target_month, from_time.day(),
    );

    from_time
        .with_day(target_day).unwrap()
        .with_month(target_month).unwrap()
        .with_year(target_year).unwrap()
}

/// Move datetime into first or last of the specified month
pub(crate) fn offset_range_month(from_time: DateTime<FixedOffset>, month: i64, change: Change) -> Result<DateTime<FixedOffset>, ()> {
    if change.eq(&Change::First) {
        return date_ymd(from_time, from_time.year() as i64, month, 1);
    }

    if change.eq(&Change::Last) {
        let last_day: u32 = into_month_day(from_time.year(), month as u32, 32);
        return date_ymd(from_time, from_time.year() as i64, month, last_day as i64);
    }

    Ok(from_time)
}

/// Move datetime into previous or upcoming weekday
pub(crate) fn offset_weekday(from_time: DateTime<FixedOffset>, new_weekday: i64, change: Change) -> DateTime<FixedOffset> {
    let curr_weekday: i64 = from_time.weekday().num_days_from_monday() as i64 + 1;

    let mut offset_weeks: i64 = 0;

    if change.eq(&Change::Prev) && curr_weekday.le(&new_weekday) {
        offset_weeks = -1;
    } else if change.eq(&Change::Next) && curr_weekday.ge(&new_weekday) {
        offset_weeks = 1;
    }

    from_time
        + Duration::weeks(offset_weeks)
        + Duration::days(new_weekday - curr_weekday)
}

/// Move datetime by given amount of weeks, to the start of the week
pub(crate) fn offset_weeks(from_time: DateTime<FixedOffset>, amount: i64, start_day: i8) -> DateTime<FixedOffset> {
    let days_since_start: i64 = match start_day {
        1 => from_time.weekday().num_days_from_monday() as i64,
        _ => from_time.weekday().num_days_from_sunday() as i64,
    };

    from_time
        - Duration::days(days_since_start)
        + Duration::weeks(amount)
}

/// Move datetime by given amount of years
pub(crate) fn offset_years(from_time: DateTime<FixedOffset>, amount: i64) -> DateTime<FixedOffset> {
    let new_year: i32 = from_time.year() + amount as i32;

    if from_time.month() != 2 {
        return from_time.with_year(new_year).unwrap();
    }

    from_time
        .with_day(1).unwrap()
        .with_year(new_year).unwrap()
        .with_day(into_month_day(new_year, 2, from_time.day())).unwrap()
}

// Move datetime into specified hour, minute and second
pub(crate) fn time_hms(from_time: DateTime<FixedOffset>, hour: i64, min: i64, sec: i64) -> Result<DateTime<FixedOffset>, ()> {
    if hour.lt(&0) || hour.gt(&23)
        || min.lt(&0) || min.gt(&59)
        || sec.lt(&0) || sec.gt(&59) {
        return Err(());
    }

    Ok(from_time
        .with_hour(hour as u32).unwrap()
        .with_minute(min as u32).unwrap()
        .with_second(sec as u32).unwrap()
        .with_nanosecond(0).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

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

        assert_eq!(
            date_ymd(from_time, 2022, 2, 25).unwrap().to_string(),
            "2022-02-25 15:22:28 +02:00",
        );

        assert_eq!(
            date_ymd(from_time, 2024, 2, 29).unwrap().to_string(),
            "2024-02-29 15:22:28 +02:00",
        );

        assert!(date_ymd(from_time, 2024, 13, 10).is_err());
        assert!(date_ymd(from_time, 2024, 2, 30).is_err());
    }

    #[test]
    fn test_into_month_day() {
        assert_eq!(into_month_day(2024, 2, 1), 1);
        assert_eq!(into_month_day(2024, 2, 29), 29);
        assert_eq!(into_month_day(2024, 2, 30), 29);
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
    fn test_offset_range_months() {
        let expect: Vec<(&str, i64, Change, &str)> = vec![
            ("2024-01-31T15:22:28+02:00", 2, Change::None, "2024-01-31 15:22:28 +02:00"),
            ("2024-01-31T15:22:28+02:00", 2, Change::First, "2024-02-01 15:22:28 +02:00"),
            ("2024-01-31T15:22:28+02:00", 2, Change::Last, "2024-02-29 15:22:28 +02:00"),
        ];

        for (from_time, new_month, change, expect_time) in expect {
            let result_time = offset_range_month(into_datetime(from_time), new_month, change);
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

        assert_eq!(
            time_hms(from_time, 0, 0, 0).unwrap().to_string(),
            "2022-02-28 00:00:00 +02:00",
        );

        assert_eq!(
            time_hms(from_time, 23, 15, 01).unwrap().to_string(),
            "2022-02-28 23:15:01 +02:00",
        );

        assert!(time_hms(from_time, -1, 0, 0).is_err());
        assert!(time_hms(from_time, 24, 0, 0).is_err());

        assert!(time_hms(from_time, 0, -1, 0).is_err());
        assert!(time_hms(from_time, 0, 60, 0).is_err());

        assert!(time_hms(from_time, 0, 0, -1).is_err());
        assert!(time_hms(from_time, 0, 0, 60).is_err());
    }

    fn into_datetime(time_str: &str) -> DateTime<FixedOffset> {
        DateTime::parse_from_rfc3339(time_str).unwrap()
    }
}