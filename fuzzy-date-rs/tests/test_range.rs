use fuzzy_date_rs::FuzzyRange;
use fuzzy_date_rs::token::WeekStartDay;

#[test]
fn test_second_ranges() {
    assert_convert_from_mon(vec![
        (
            "last 30 seconds",
            "2024-01-25T15:22:28.123+02:00",
            "2024-01-25 15:21:58 +02:00",
            "2024-01-25 15:22:29 +02:00",
        ),
    ]);
}

#[test]
fn test_minute_ranges() {
    assert_convert_from_mon(vec![
        ("this minute", "2024-01-25T15:22:28.256+02:00", "2024-01-25 15:22:00 +02:00", "2024-01-25 15:23:00 +02:00"),
        ("last minute", "2024-01-25T15:22:28+02:00", "2024-01-25 15:21:00 +02:00", "2024-01-25 15:22:00 +02:00"),
        ("next minute", "2024-01-25T15:22:28+02:00", "2024-01-25 15:23:00 +02:00", "2024-01-25 15:24:00 +02:00"),
    ]);
}

#[test]
fn test_hour_ranges() {
    assert_convert_from_mon(vec![
        ("this hour", "2024-01-25T15:22:28.256+02:00", "2024-01-25 15:00:00 +02:00", "2024-01-25 16:00:00 +02:00"),
        ("last hour", "2024-01-25T15:22:28+02:00", "2024-01-25 14:00:00 +02:00", "2024-01-25 15:00:00 +02:00"),
        ("next hour", "2024-01-25T15:22:28+02:00", "2024-01-25 16:00:00 +02:00", "2024-01-25 17:00:00 +02:00"),
        ("last 24 hours", "2024-01-25T15:22:28+02:00", "2024-01-24 15:00:00 +02:00", "2024-01-25 15:00:00 +02:00"),
    ]);
}

#[test]
fn test_day_ranges() {
    assert_convert_from_mon(vec![
        ("today", "2024-01-25T15:22:28.256+02:00", "2024-01-25 00:00:00 +02:00", "2024-01-26 00:00:00 +02:00"),
        ("yesterday", "2024-01-25T15:22:28+02:00", "2024-01-24 00:00:00 +02:00", "2024-01-25 00:00:00 +02:00"),
        ("tomorrow", "2024-01-25T15:22:28+02:00", "2024-01-26 00:00:00 +02:00", "2024-01-27 00:00:00 +02:00"),
        ("last 2 days", "2024-01-25T15:22:28+02:00", "2024-01-23 00:00:00 +02:00", "2024-01-25 00:00:00 +02:00"),
    ]);
}

#[test]
fn test_week_ranges_monday() {
    assert_convert_from_mon(vec![
        ("this week", "2024-01-25T15:22:28.256+02:00", "2024-01-22 00:00:00 +02:00", "2024-01-26 00:00:00 +02:00"),
        ("prev week", "2024-01-25T15:22:28+02:00", "2024-01-15 00:00:00 +02:00", "2024-01-22 00:00:00 +02:00"),
        ("next week", "2024-01-13T15:22:28+02:00", "2024-01-15 00:00:00 +02:00", "2024-01-22 00:00:00 +02:00"),
        ("prev 2 weeks", "2024-01-25T15:22:28+02:00", "2024-01-08 00:00:00 +02:00", "2024-01-22 00:00:00 +02:00"),
    ]);
}

#[test]
fn test_week_ranges_sunday() {
    assert_convert_from_sun(vec![
        ("this week", "2024-01-25T15:22:28+02:00", "2024-01-21 00:00:00 +02:00", "2024-01-26 00:00:00 +02:00"),
        ("prev week", "2024-01-25T15:22:28+02:00", "2024-01-14 00:00:00 +02:00", "2024-01-21 00:00:00 +02:00"),
        ("next week", "2024-01-13T15:22:28+02:00", "2024-01-14 00:00:00 +02:00", "2024-01-21 00:00:00 +02:00"),
        ("prev 2 weeks", "2024-01-25T15:22:28+02:00", "2024-01-07 00:00:00 +02:00", "2024-01-21 00:00:00 +02:00"),
    ]);
}

#[test]
fn test_month_ranges() {
    assert_convert_from_mon(vec![
        ("this month", "2024-03-12T15:22:28+02:00", "2024-03-01 00:00:00 +02:00", "2024-03-13 00:00:00 +02:00"),
        ("prev month", "2024-03-12T15:22:28+02:00", "2024-02-01 00:00:00 +02:00", "2024-03-01 00:00:00 +02:00"),
        ("next month", "2024-12-12T15:22:28+02:00", "2025-01-01 00:00:00 +02:00", "2025-02-01 00:00:00 +02:00"),
        ("last 2 months", "2024-03-12T15:22:28+02:00", "2024-01-01 00:00:00 +02:00", "2024-03-01 00:00:00 +02:00"),
    ]);
}

#[test]
fn test_year_ranges() {
    assert_convert_from_mon(vec![
        ("this year", "2024-03-12T15:22:28+02:00", "2024-01-01 00:00:00 +02:00", "2024-03-13 00:00:00 +02:00"),
        ("last year", "2024-03-12T15:22:28+02:00", "2023-01-01 00:00:00 +02:00", "2024-01-01 00:00:00 +02:00"),
        ("next year", "2024-12-12T15:22:28+02:00", "2025-01-01 00:00:00 +02:00", "2026-01-01 00:00:00 +02:00"),
        ("last 2 years", "2024-03-12T15:22:28+02:00", "2022-01-01 00:00:00 +02:00", "2024-01-01 00:00:00 +02:00"),
    ]);
}

fn assert_convert_from_mon(expect: Vec<(&str, &str, &str, &str)>) {
    for (from_string, current_time, expect_start, expect_end) in expect {
        let result_time = FuzzyRange::from_rfc3339(current_time)
            .set_first_weekday(WeekStartDay::Monday)
            .to_range(from_string);

        assert_eq!(result_time.unwrap().0.to_string(), expect_start.to_string());
        assert_eq!(result_time.unwrap().1.to_string(), expect_end.to_string());
    }
}

fn assert_convert_from_sun(expect: Vec<(&str, &str, &str, &str)>) {
    for (from_string, current_time, expect_start, expect_end) in expect {
        let result_time = FuzzyRange::from_rfc3339(current_time)
            .set_first_weekday(WeekStartDay::Sunday)
            .to_range(from_string);

        assert_eq!(result_time.unwrap().0.to_string(), expect_start.to_string());
        assert_eq!(result_time.unwrap().1.to_string(), expect_end.to_string());
    }
}
