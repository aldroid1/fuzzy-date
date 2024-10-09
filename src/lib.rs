mod convert;
mod fuzzy;
mod token;
mod python;

use chrono::{DateTime, FixedOffset, NaiveDate};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyDate, PyDateTime};

/// Turn time string into Python's datetime.date
#[pyfunction]
#[pyo3(signature = (source, today=None, weekday_start_mon=true))]
fn to_date(
    py: Python,
    source: &str,
    today: Option<Py<PyDate>>,
    weekday_start_mon: bool) -> PyResult<NaiveDate> {
    match convert_str(&source, &python::into_date(py, today)?, weekday_start_mon) {
        Some(v) => Ok(v.date_naive()),
        None => Err(PyValueError::new_err(format!(
            "Unable to convert \"{}\" into datetime", source,
        )))
    }
}

/// Turn time string into Python's datetime.datetime
#[pyfunction]
#[pyo3(signature = (source, now=None, weekday_start_mon=true))]
fn to_datetime(
    py: Python,
    source: &str,
    now: Option<Py<PyDateTime>>,
    weekday_start_mon: bool) -> PyResult<DateTime<FixedOffset>> {
    match convert_str(&source, &python::into_datetime(py, now)?, weekday_start_mon) {
        Some(v) => Ok(v),
        None => Err(PyValueError::new_err(format!(
            "Unable to convert \"{}\" into datetime", source,
        )))
    }
}

/// Time string conversion module for Python
#[pymodule]
fn fuzzydate(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(to_date, m)?)?;
    m.add_function(wrap_pyfunction!(to_datetime, m)?)?;
    Ok(())
}

/// Tokenize source string and then convert it into a datetime value
fn convert_str(
    source: &str,
    current_time: &DateTime<FixedOffset>,
    first_weekday_mon: bool) -> Option<DateTime<FixedOffset>> {
    let (pattern, tokens) = token::tokenize(&source);
    let values: Vec<i64> = tokens.into_iter().map(|p| p.value).collect();
    fuzzy::convert(&pattern, &values, &current_time, first_weekday_mon)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_fixed_dates() {
        let expect: Vec<(&str, &str)> = vec![
            ("@1705072948", "2024-01-12 15:22:28 +00:00"),
            ("@1705072948.0", "2024-01-12 15:22:28 +00:00"),
            ("@1705072948.544", "2024-01-12 15:22:28.544 +00:00"),
            ("2023-01-01", "2023-01-01 00:00:00 +00:00"),
            ("07.02.2023", "2023-02-07 00:00:00 +00:00"),
            ("7.2.2023", "2023-02-07 00:00:00 +00:00"),
            ("2/7/2023", "2023-02-07 00:00:00 +00:00"),
            ("Dec 7 2023", "2023-12-07 00:00:00 +00:00"),
            ("Dec 7th 2023", "2023-12-07 00:00:00 +00:00"),
            ("December 7th 2023", "2023-12-07 00:00:00 +00:00"),
            ("7 Dec 2023", "2023-12-07 00:00:00 +00:00"),
            ("7 December 2023", "2023-12-07 00:00:00 +00:00"),
            ("2023-12-07 15:02", "2023-12-07 15:02:00 +00:00"),
            ("2023-12-07 15:02:01", "2023-12-07 15:02:01 +00:00"),
        ];

        let current_time = Utc::now().fixed_offset();

        for (from_string, expect_time) in expect {
            let result_time = convert_str(from_string, &current_time, true);
            assert_eq!(result_time.unwrap().to_string(), expect_time.to_string());
        }
    }

    #[test]
    fn test_keywords() {
        assert_convert_from(vec![
            ("now", "2024-01-12T15:22:28+02:00", "2024-01-12 15:22:28 +02:00"),
            ("midnight", "2024-01-12T15:22:28+02:00", "2024-01-12 00:00:00 +02:00"),
            ("yesterday", "2024-01-12T15:22:28+02:00", "2024-01-11 15:22:28 +02:00"),
            ("tomorrow", "2024-01-12T15:22:28+02:00", "2024-01-13 15:22:28 +02:00"),
        ]);
    }

    #[test]
    fn test_month_ranges() {
        assert_convert_from(vec![
            // First
            ("first day of January", "2024-05-12T15:22:28+02:00", "2024-01-01 00:00:00 +02:00"),
            ("first day of this month", "2024-02-12T15:22:28+02:00", "2024-02-01 00:00:00 +02:00"),
            ("first day of prev month", "2024-03-12T15:22:28+02:00", "2024-02-01 00:00:00 +02:00"),
            ("first day of last month", "2024-03-12T15:22:28+02:00", "2024-02-01 00:00:00 +02:00"),
            ("first day of next month", "2024-02-12T15:22:28+02:00", "2024-03-01 00:00:00 +02:00"),

            // Last
            ("last day of February", "2024-05-12T15:22:28+02:00", "2024-02-29 00:00:00 +02:00"),
            ("last day of this month", "2024-02-12T15:22:28+02:00", "2024-02-29 00:00:00 +02:00"),
            ("last day of prev month", "2024-03-12T15:22:28+02:00", "2024-02-29 00:00:00 +02:00"),
            ("last day of last month", "2024-03-12T15:22:28+02:00", "2024-02-29 00:00:00 +02:00"),
            ("last day of next month", "2023-12-12T15:22:28+02:00", "2024-01-31 00:00:00 +02:00"),
        ]);
    }

    #[test]
    fn test_offset_seconds() {
        assert_convert_from(vec![
            ("this second", "2024-01-12T15:22:28+02:00", "2024-01-12 15:22:28 +02:00"),
            ("prev second", "2024-01-12T15:22:28+02:00", "2024-01-12 15:22:27 +02:00"),
            ("last second", "2024-01-12T15:22:28+02:00", "2024-01-12 15:22:27 +02:00"),
            ("next second", "2024-01-12T15:22:28+02:00", "2024-01-12 15:22:29 +02:00"),
            ("-1s", "2024-01-12T15:22:28+02:00", "2024-01-12 15:22:27 +02:00"),
            ("-1sec", "2024-01-12T15:22:28+02:00", "2024-01-12 15:22:27 +02:00"),
            ("-1 second", "2024-01-12T15:22:28+02:00", "2024-01-12 15:22:27 +02:00"),
            ("+1s", "2024-01-12T15:22:28+02:00", "2024-01-12 15:22:29 +02:00"),
            ("+1sec", "2024-01-12T15:22:28+02:00", "2024-01-12 15:22:29 +02:00"),
            ("+60 seconds", "2024-01-12T15:22:28+02:00", "2024-01-12 15:23:28 +02:00"),
            ("1 sec ago", "2024-01-25T15:22:28+02:00", "2024-01-25 15:22:27 +02:00"),
            ("1 seconds ago", "2024-01-25T15:22:28+02:00", "2024-01-25 15:22:27 +02:00"),
        ]);
    }

    #[test]
    fn test_offset_minutes() {
        assert_convert_from(vec![
            ("this minute", "2024-01-12T15:22:28+02:00", "2024-01-12 15:22:28 +02:00"),
            ("prev minute", "2024-01-12T15:22:28+02:00", "2024-01-12 15:21:28 +02:00"),
            ("last minute", "2024-01-12T15:22:28+02:00", "2024-01-12 15:21:28 +02:00"),
            ("next minute", "2024-01-12T15:22:28+02:00", "2024-01-12 15:23:28 +02:00"),
            ("-1min", "2024-01-12T15:22:28+02:00", "2024-01-12 15:21:28 +02:00"),
            ("-5 minutes", "2024-01-12 15:22:28+02:00", "2024-01-12 15:17:28 +02:00"),
            ("+60min", "2024-01-12T15:22:28+02:00", "2024-01-12 16:22:28 +02:00"),
            ("+60 minutes", "2024-01-12T15:22:28+02:00", "2024-01-12 16:22:28 +02:00"),
            ("1 min ago", "2024-01-12T15:22:28+02:00", "2024-01-12 15:21:28 +02:00"),
            ("5 minutes ago", "2024-01-12 15:22:28+02:00", "2024-01-12 15:17:28 +02:00"),
        ]);
    }

    #[test]
    fn test_offset_hours() {
        assert_convert_from(vec![
            ("this hour", "2024-01-12T15:22:28+02:00", "2024-01-12 15:22:28 +02:00"),
            ("prev hour", "2024-01-12T15:22:28+02:00", "2024-01-12 14:22:28 +02:00"),
            ("last hour", "2024-01-12T15:22:28+02:00", "2024-01-12 14:22:28 +02:00"),
            ("next hour", "2024-01-12T15:22:28+02:00", "2024-01-12 16:22:28 +02:00"),
            ("-1h", "2024-01-12T15:22:28+02:00", "2024-01-12 14:22:28 +02:00"),
            ("-1hr", "2024-01-12T15:22:28+02:00", "2024-01-12 14:22:28 +02:00"),
            ("-1 hour", "2024-01-12T15:22:28+02:00", "2024-01-12 14:22:28 +02:00"),
            ("+1h", "2024-01-12T15:22:28+02:00", "2024-01-12 16:22:28 +02:00"),
            ("+1hr", "2024-01-12T15:22:28+02:00", "2024-01-12 16:22:28 +02:00"),
            ("+30 hours", "2024-01-12T15:22:28+02:00", "2024-01-13 21:22:28 +02:00"),
            ("1 hr ago", "2024-01-12T15:22:28+02:00", "2024-01-12 14:22:28 +02:00"),
            ("1 hour ago", "2024-01-12T15:22:28+02:00", "2024-01-12 14:22:28 +02:00"),
        ]);
    }

    #[test]
    fn test_offset_days() {
        assert_convert_from(vec![
            ("this day", "2024-01-12T15:22:28+02:00", "2024-01-12 15:22:28 +02:00"),
            ("prev day", "2024-01-12T15:22:28+02:00", "2024-01-11 15:22:28 +02:00"),
            ("last day", "2024-01-12T15:22:28+02:00", "2024-01-11 15:22:28 +02:00"),
            ("next day", "2024-01-12T15:22:28+02:00", "2024-01-13 15:22:28 +02:00"),
            ("-1d", "2024-01-12T15:22:28+02:00", "2024-01-11 15:22:28 +02:00"),
            ("-1 day", "2024-01-12T15:22:28+02:00", "2024-01-11 15:22:28 +02:00"),
            ("+1d", "2024-01-12T15:22:28+02:00", "2024-01-13 15:22:28 +02:00"),
            ("+30 days", "2024-01-12T15:22:28+02:00", "2024-02-11 15:22:28 +02:00"),
            ("2 days ago", "2024-01-12T15:22:28+02:00", "2024-01-10 15:22:28 +02:00"),
        ]);
    }

    #[test]
    fn test_offset_weekdays() {
        assert_convert_from(vec![
            ("this Sunday", "2024-01-19T15:22:28+02:00", "2024-01-21 15:22:28 +02:00"),
            ("prev Sunday", "2024-01-19T15:22:28+02:00", "2024-01-14 15:22:28 +02:00"),
            ("last Mon", "2024-01-19T15:22:28+02:00", "2024-01-15 15:22:28 +02:00"),
            ("next Mon", "2024-01-19T15:22:28+02:00", "2024-01-22 15:22:28 +02:00"),
            ("next Sunday", "2024-01-19T15:22:28+02:00", "2024-01-21 15:22:28 +02:00"),

            // Current weekday is the same as new weekday
            ("this Saturday", "2024-01-20T15:22:28+02:00", "2024-01-20 15:22:28 +02:00"),
            ("prev Saturday", "2024-01-20T15:22:28+02:00", "2024-01-13 15:22:28 +02:00"),
            ("next Saturday", "2024-01-20T15:22:28+02:00", "2024-01-27 15:22:28 +02:00"),
        ]);
    }

    #[test]
    fn test_offset_weeks_monday() {
        let expect: Vec<(&str, &str, &str)> = vec![
            ("this week", "2024-01-25T15:22:28+02:00", "2024-01-22 15:22:28 +02:00"),
            ("prev week", "2024-01-25T15:22:28+02:00", "2024-01-15 15:22:28 +02:00"),
            ("last week", "2024-01-25T15:22:28+02:00", "2024-01-15 15:22:28 +02:00"),
            ("next week", "2024-01-13 15:22:28+02:00", "2024-01-15 15:22:28 +02:00"),
            ("-1w", "2024-01-25T15:22:28+02:00", "2024-01-15 15:22:28 +02:00"),
            ("-2 weeks", "2024-01-25T15:22:28+02:00", "2024-01-08 15:22:28 +02:00"),
            ("+1w", "2024-01-14T14:22:28+02:00", "2024-01-15 14:22:28 +02:00"),
            ("+2 weeks", "2024-01-08T15:22:28+02:00", "2024-01-22 15:22:28 +02:00"),
            ("1 week ago", "2024-01-25T15:22:28+02:00", "2024-01-15 15:22:28 +02:00"),
        ];

        for (from_string, current_time, expect_time) in expect {
            let current_time = DateTime::parse_from_rfc3339(current_time).unwrap();
            let result_time = convert_str(from_string, &current_time, true);
            assert_eq!(result_time.unwrap().to_string(), expect_time.to_string())
        }
    }

    #[test]
    fn test_offset_weeks_sunday() {
        let expect: Vec<(&str, &str, &str)> = vec![
            ("this week", "2024-01-25T15:22:28+02:00", "2024-01-21 15:22:28 +02:00"),
            ("prev week", "2024-01-25T15:22:28+02:00", "2024-01-14 15:22:28 +02:00"),
            ("last week", "2024-01-25T15:22:28+02:00", "2024-01-14 15:22:28 +02:00"),
            ("next week", "2024-01-13 15:22:28+02:00", "2024-01-14 15:22:28 +02:00"),
            ("-1w", "2024-01-25T15:22:28+02:00", "2024-01-14 15:22:28 +02:00"),
            ("-2 weeks", "2024-01-25T15:22:28+02:00", "2024-01-07 15:22:28 +02:00"),
            ("+1w", "2024-01-14T14:22:28+02:00", "2024-01-21 14:22:28 +02:00"),
            ("+2 weeks", "2024-01-08T15:22:28+02:00", "2024-01-21 15:22:28 +02:00"),
            ("1 week ago", "2024-01-25T15:22:28+02:00", "2024-01-14 15:22:28 +02:00"),
        ];

        for (from_string, current_time, expect_time) in expect {
            let current_time = DateTime::parse_from_rfc3339(current_time).unwrap();
            let result_time = convert_str(from_string, &current_time, false);
            assert_eq!(result_time.unwrap().to_string(), expect_time.to_string())
        }
    }

    #[test]
    fn test_offset_months() {
        assert_convert_from(vec![
            ("this month", "2024-03-12T15:22:28+02:00", "2024-03-12 15:22:28 +02:00"),
            ("prev month", "2024-03-12T15:22:28+02:00", "2024-02-12 15:22:28 +02:00"),
            ("last month", "2024-03-12T15:22:28+02:00", "2024-02-12 15:22:28 +02:00"),
            ("next month", "2024-12-12T15:22:28+02:00", "2025-01-12 15:22:28 +02:00"),
            ("-1m", "2024-03-12T15:22:28+02:00", "2024-02-12 15:22:28 +02:00"),
            ("-1 month", "2024-03-12T15:22:28+02:00", "2024-02-12 15:22:28 +02:00"),
            ("+1m", "2024-03-12T15:22:28+02:00", "2024-04-12 15:22:28 +02:00"),
            ("+13 months", "2023-12-12T15:22:28+02:00", "2025-01-12 15:22:28 +02:00"),
            ("1 month ago", "2024-03-12T15:22:28+02:00", "2024-02-12 15:22:28 +02:00"),

            // Different number of days in each month
            ("-1m", "2022-05-31T15:22:28+02:00", "2022-04-30 15:22:28 +02:00"),
        ]);
    }

    #[test]
    fn test_offset_years() {
        assert_convert_from(vec![
            ("this year", "2024-01-12T15:22:28+02:00", "2024-01-12 15:22:28 +02:00"),
            ("prev year", "2024-01-12T15:22:28+02:00", "2023-01-12 15:22:28 +02:00"),
            ("last year", "2024-01-12T15:22:28+02:00", "2023-01-12 15:22:28 +02:00"),
            ("next year", "2024-01-12T15:22:28+02:00", "2025-01-12 15:22:28 +02:00"),
            ("-1y", "2024-01-12T15:22:28+02:00", "2023-01-12 15:22:28 +02:00"),
            ("-1 year", "2024-01-12T15:22:28+02:00", "2023-01-12 15:22:28 +02:00"),
            ("+1y", "2024-01-12T15:22:28+02:00", "2025-01-12 15:22:28 +02:00"),
            ("+10 years", "2024-01-12T15:22:28+02:00", "2034-01-12 15:22:28 +02:00"),
            ("2 years ago", "2024-01-12T15:22:28+02:00", "2022-01-12 15:22:28 +02:00"),

            // Non-leap years
            ("-1y", "2022-02-01T15:22:28+02:00", "2021-02-01 15:22:28 +02:00"),
            ("-1y", "2022-02-05T15:22:28+02:00", "2021-02-05 15:22:28 +02:00"),
            ("-1y", "2022-02-28T15:22:28+02:00", "2021-02-28 15:22:28 +02:00"),

            // Leap year
            ("-1y", "2024-02-29T15:22:28+02:00", "2023-02-28 15:22:28 +02:00"),
        ]);
    }

    #[test]
    fn test_combinations() {
        assert_convert_from(vec![
            ("yesterday midnight", "2024-01-12T15:22:28+02:00", "2024-01-11 00:00:00 +02:00"),
            ("-2d 1h", "2024-05-12T15:22:28+02:00", "2024-05-10 14:22:28 +02:00"),
            ("-2d 1h midnight", "2024-05-12T15:22:28+02:00", "2024-05-10 00:00:00 +02:00"),
            ("first day of Jan last year", "2024-05-12T15:22:28+02:00", "2023-01-01 00:00:00 +02:00"),
            ("last day of Feb last year", "2024-05-12T15:22:28+02:00", "2023-02-28 00:00:00 +02:00"),
        ]);
    }

    #[test]
    fn test_unsupported() {
        let expect: Vec<&str> = vec![
            "",                       // Not parsed
            " ",                      // Nothing to parse
            "+1day",                  // Not recognized
            "0000-01-12 15:22",       // Year invalid
            "1982-04-32",             // Date invalid
            "1982-04-01 15:61",       // Time invalid
            "Feb 29th 2023",          // Day out of range
            "first day of this week", // Not supported
            "first minute of Jan",    // Not supported
        ];

        let current_time = Utc::now().fixed_offset();

        for from_string in expect {
            let result_time = convert_str(from_string, &current_time, true);
            assert!(result_time.is_none());
        }
    }

    fn assert_convert_from(expect: Vec<(&str, &str, &str)>) {
        for (from_string, current_time, expect_time) in expect {
            let current_time = DateTime::parse_from_rfc3339(current_time).unwrap();
            let result_time = convert_str(from_string, &current_time, false);
            assert_eq!(result_time.unwrap().to_string(), expect_time.to_string());
        }
    }
}

