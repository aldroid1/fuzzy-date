use fuzzy_date_rs::{FuzzyDuration, FuzzySeconds};
use fuzzy_date_rs::token::UnitSet;

#[test]
fn test_to_duration_all() {
    assert_to_duration(
        "",
        "",
        vec![
            // Short
            (0.0, "short", ""),
            (604800.0, "short", "1w"),
            (1209600.0, "short", "2w"),
            (0.0, "short", ""),
            (86400.0, "short", "1d"),
            (172800.0, "short", "2d"),
            (0.0, "short", ""),
            (3600.0, "short", "1h"),
            (7200.0, "short", "2h"),
            (0.0, "short", ""),
            (60.0, "short", "1min"),
            (120.0, "short", "2min"),
            (0.0, "short", ""),
            (1.0, "short", "1s"),
            (2.0, "short", "2s"),
            // Long
            (0.0, "long", ""),
            (604800.0, "long", "1 week"),
            (1209600.0, "long", "2 weeks"),
            (0.0, "long", ""),
            (86400.0, "long", "1 day"),
            (172800.0, "long", "2 days"),
            (0.0, "long", ""),
            (3600.0, "long", "1 hour"),
            (7200.0, "long", "2 hours"),
            (0.0, "long", ""),
            (60.0, "long", "1 minute"),
            (120.0, "long", "2 minutes"),
            (0.0, "long", ""),
            (1.0, "long", "1 second"),
            (2.0, "long", "2 seconds"),
            // Default
            (0.0, "", ""),
            (604800.0, "", "1w"),
            (1209600.0, "", "2w"),
            (0.0, "", ""),
            (86400.0, "", "1d"),
            (172800.0, "", "2d"),
            (0.0, "", ""),
            (3600.0, "", "1hr"),
            (7200.0, "", "2hrs"),
            (0.0, "", ""),
            (60.0, "", "1min"),
            (120.0, "", "2min"),
            (0.0, "", ""),
            (1.0, "", "1sec"),
            (2.0, "", "2sec"),
            // Combinations
            (694861.0, "", "1w 1d 1hr 1min 1sec"),
            (1389722.0, "", "2w 2d 2hrs 2min 2sec"),
            (1389720.0, "", "2w 2d 2hrs 2min"),
            (1389600.0, "", "2w 2d 2hrs"),
            (1382400.0, "", "2w 2d"),
            (1209600.0, "", "2w"),
        ],
    )
}

#[test]
fn test_to_duration_min_max() {
    assert_to_duration("w", "d", vec![(694800.0, "short", "1w 1d")]);
    assert_to_duration("d", "d", vec![(694800.0, "short", "8d")]);
    assert_to_duration("d", "h", vec![(694800.0, "short", "8d 1h")]);
    assert_to_duration("d", "s", vec![(694800.0, "short", "8d 1h")]);
    assert_to_duration("h", "h", vec![(694800.0, "short", "193h")]);
    assert_to_duration("min", "s", vec![(695165.0, "short", "11586min 5s")]);
    assert_to_duration("h", "s", vec![(695165.0, "short", "193h 6min 5s")]);
    assert_to_duration("s", "s", vec![(695165.0, "short", "695165s")]);
}

#[test]
fn test_to_seconds_some() {
    let expect: Vec<(&str, f64)> = vec![
        ("1 day", 86400.0),
        ("1d", 86400.0),
        ("-1 day", -86400.0),
        ("1 hour", 3600.0),
        ("1h", 3600.0),
        ("-1 hour", -3600.0),
        ("1d 1h 1min 2s", 90062.0),
        ("+1d 1h 1min 2s", 90062.0),
        ("-1d 1h 1min 2s", -90062.0),
        ("1d 1h 1min -2s", 90058.0),
        ("-1d 1h 1min +2s", -90058.0),
        ("-1d +1h -1min", -82860.0),
    ];

    for (from_string, expect_value) in expect {
        let result_value = FuzzySeconds::new().to_seconds(from_string);
        assert_eq!(result_value.unwrap(), expect_value);
    }
}

#[test]
fn test_to_seconds_none() {
    let expect: Vec<&str> = vec![
        "",
        "7",
        "2020-01-07",
        "last week",
        "past week",
        "1 hour ago",
        "1y",
        "+1 year",
        "-2 years",
        "1m",
        "+1 month",
        "-2 months",
    ];

    for from_string in expect {
        let result_value = FuzzySeconds::new().to_seconds(from_string);
        assert!(result_value.is_err());
    }
}

fn assert_to_duration(max: &str, min: &str, expect: Vec<(f64, &str, &str)>) {
    for (from_seconds, unit_types, expect_str) in expect {
        let into_duration = FuzzyDuration::new()
            .set_default_units(UnitSet::from_str(unit_types))
            .set_min_unit(min)
            .set_max_unit(max)
            .to_duration(from_seconds);

        assert_eq!(into_duration, expect_str);

        if max.eq("") && min.eq("") && expect_str.len().gt(&0) {
            let into_seconds = FuzzySeconds::new().to_seconds(expect_str);
            assert_eq!(into_seconds.unwrap(), from_seconds);
        }
    }
}
