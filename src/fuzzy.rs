use crate::constants::Pattern;
use crate::convert;
use chrono::{DateTime, Datelike, Duration, FixedOffset};
use std::cmp;
use std::cmp::PartialEq;
use std::collections::HashMap;

const FUZZY_PATTERNS: [(&Pattern, fn(FuzzyDate, &Vec<i64>, &Rules) -> Result<FuzzyDate, ()>); 44] = [
    // KEYWORDS
    (&Pattern::Now, |c, _, _| Ok(c)),
    (&Pattern::Today, |c, _, _| c.time_reset()),
    (&Pattern::Midnight, |c, _, _| c.time_reset()),
    (&Pattern::Yesterday, |c, _, r| c.offset_unit_keyword(TimeUnit::Days, -1, r)?.time_reset()),
    (&Pattern::Tomorrow, |c, _, r| c.offset_unit_keyword(TimeUnit::Days, 1, r)?.time_reset()),

    // WEEKDAY OFFSETS
    (&Pattern::ThisWday, |c, v, _| c.offset_weekday(v[0], convert::Change::None)?.time_reset()),
    (&Pattern::PrevWday, |c, v, _| c.offset_weekday(v[0], convert::Change::Prev)?.time_reset()),
    (&Pattern::LastWday, |c, v, _| c.offset_weekday(v[0], convert::Change::Prev)?.time_reset()),
    (&Pattern::NextWday, |c, v, _| c.offset_weekday(v[0], convert::Change::Next)?.time_reset()),

    // KEYWORD OFFSETS
    (&Pattern::ThisLongUnit, |c, v, r| c.offset_unit_keyword(TimeUnit::from_int(v[0]), 0, r)),
    (&Pattern::PrevLongUnit, |c, v, r| c.offset_unit_keyword(TimeUnit::from_int(v[0]), -1, r)),
    (&Pattern::LastLongUnit, |c, v, r| c.offset_unit_keyword(TimeUnit::from_int(v[0]), -1, r)),
    (&Pattern::NextLongUnit, |c, v, r| c.offset_unit_keyword(TimeUnit::from_int(v[0]), 1, r)),

    // NUMERIC OFFSET, MINUS
    (&Pattern::MinusUnit, |c, v, r| c.offset_unit_exact(TimeUnit::from_int(v[1]), 0 - v[0], r)),
    (&Pattern::MinusShortUnit, |c, v, r| c.offset_unit_exact(TimeUnit::from_int(v[1]), 0 - v[0], r)),
    (&Pattern::MinusLongUnit, |c, v, r| c.offset_unit_exact(TimeUnit::from_int(v[1]), 0 - v[0], r)),

    // NUMERIC OFFSET, PLUS
    (&Pattern::PlusUnit, |c, v, r| c.offset_unit_exact(TimeUnit::from_int(v[1]), v[0], r)),
    (&Pattern::PlusShortUnit, |c, v, r| c.offset_unit_exact(TimeUnit::from_int(v[1]), v[0], r)),
    (&Pattern::PlusLongUnit, |c, v, r| c.offset_unit_exact(TimeUnit::from_int(v[1]), v[0], r)),

    // NUMERIC OFFSET, PLUS
    (&Pattern::UnitAgo, |c, v, r| c.offset_unit_exact(TimeUnit::from_int(v[1]), 0 - v[0], r)),
    (&Pattern::LongUnitAgo, |c, v, r| c.offset_unit_exact(TimeUnit::from_int(v[1]), 0 - v[0], r)),

    // FIRST/LAST OFFSETS
    (&Pattern::FirstLongUnitOfMonth, |c, v, _| c
        .offset_range_month(TimeUnit::from_int(v[0]), v[1], convert::Change::First)?
        .time_reset(),
    ),
    (&Pattern::LastLongUnitOfMonth, |c, v, _| c
        .offset_range_month(TimeUnit::from_int(v[0]), v[1], convert::Change::Last)?
        .time_reset(),
    ),
    (&Pattern::FirstLongUnitOfThisLongUnit, |c, v, _| c
        .offset_range_unit(TimeUnit::from_int(v[0]), TimeUnit::from_int(v[1]), convert::Change::First)?
        .time_reset(),
    ),
    (&Pattern::LastLongUnitOfThisLongUnit, |c, v, _| c
        .offset_range_unit(TimeUnit::from_int(v[0]), TimeUnit::from_int(v[1]), convert::Change::Last)?
        .time_reset(),
    ),
    (&Pattern::FirstLongUnitOfPrevLongUnit, |c, v, r| c
        .offset_unit_keyword(TimeUnit::from_int(v[1]), -1, r)?
        .offset_range_unit(TimeUnit::from_int(v[0]), TimeUnit::from_int(v[1]), convert::Change::First)?
        .time_reset(),
    ),
    (&Pattern::LastLongUnitOfPrevLongUnit, |c, v, r| c
        .offset_unit_keyword(TimeUnit::from_int(v[1]), -1, r)?
        .offset_range_unit(TimeUnit::from_int(v[0]), TimeUnit::from_int(v[1]), convert::Change::Last)?
        .time_reset(),
    ),
    (&Pattern::FirstLongUnitOfLastLongUnit, |c, v, r| c
        .offset_unit_keyword(TimeUnit::from_int(v[1]), -1, r)?
        .offset_range_unit(TimeUnit::from_int(v[0]), TimeUnit::from_int(v[1]), convert::Change::First)?
        .time_reset(),
    ),
    (&Pattern::LastLongUnitOfLastLongUnit, |c, v, r| c
        .offset_unit_keyword(TimeUnit::from_int(v[1]), -1, r)?
        .offset_range_unit(TimeUnit::from_int(v[0]), TimeUnit::from_int(v[1]), convert::Change::Last)?
        .time_reset(),
    ),
    (&Pattern::FirstLongUnitOfNextLongUnit, |c, v, r| c
        .offset_unit_keyword(TimeUnit::from_int(v[1]), 1, r)?
        .offset_range_unit(TimeUnit::from_int(v[0]), TimeUnit::from_int(v[1]), convert::Change::First)?
        .time_reset(),
    ),
    (&Pattern::LastLongUnitOfNextLongUnit, |c, v, r| c
        .offset_unit_keyword(TimeUnit::from_int(v[1]), 1, r)?
        .offset_range_unit(TimeUnit::from_int(v[0]), TimeUnit::from_int(v[1]), convert::Change::Last)?
        .time_reset(),
    ),

    // @1705072948, @1705072948.452
    (&Pattern::Timestamp, |c, v, _| c.date_stamp(v[0], 0)),
    (&Pattern::TimestampFloat, |c, v, _| c.date_stamp(v[0], v[1])),

    // 2023-01-01, 30.1.2023, 1/30/2023
    (&Pattern::DateYmd, |c, v, _| c.date_ymd(v[0], v[1], v[2])?.time_reset()),
    (&Pattern::DateDmy, |c, v, _| c.date_ymd(v[2], v[1], v[0])?.time_reset()),
    (&Pattern::DateMdy, |c, v, _| c.date_ymd(v[2], v[0], v[1])?.time_reset()),

    // Dec 7, Dec 7th, 7 Dec
    (&Pattern::DateMonthDay, |c, v, _| c.date_ymd(c.year(), v[0], v[1])?.time_reset()),
    (&Pattern::DateMonthNth, |c, v, _| c.date_ymd(c.year(), v[0], v[1])?.time_reset()),
    (&Pattern::DateDayMonth, |c, v, _| c.date_ymd(c.year(), v[1], v[0])?.time_reset()),

    // Dec 7 2023, Dec 7th 2023, 7 Dec 2023
    (&Pattern::DateMonthDayYear, |c, v, _| c.date_ymd(v[2], v[0], v[1])?.time_reset()),
    (&Pattern::DateMonthNthYear, |c, v, _| c.date_ymd(v[2], v[0], v[1])?.time_reset()),
    (&Pattern::DateDayMonthYear, |c, v, _| c.date_ymd(v[2], v[1], v[0])?.time_reset()),

    // 2023-12-07 15:02, 2023-12-07 15:02:01
    (&Pattern::DateTimeYmdHm, |c, v, _| c.date_ymd(v[0], v[1], v[2])?.time_hms(v[3], v[4], 0)),
    (&Pattern::DateTimeYmdHms, |c, v, _| c.date_ymd(v[0], v[1], v[2])?.time_hms(v[3], v[4], v[5])),
];

#[derive(PartialEq)]
enum TimeUnit {
    Days,
    Hours,
    Minutes,
    Months,
    Seconds,
    Weeks,
    Years,
    None,
}

impl TimeUnit {
    fn from_int(value: i64) -> TimeUnit {
        match value {
            1 => Self::Seconds,
            2 => Self::Minutes,
            3 => Self::Hours,
            4 => Self::Days,
            5 => Self::Weeks,
            6 => Self::Months,
            7 => Self::Years,
            _ => Self::None,
        }
    }
}

struct FuzzyDate {
    time: DateTime<FixedOffset>,
}

impl FuzzyDate {
    fn new(time: DateTime<FixedOffset>) -> Self {
        FuzzyDate { time: time }
    }

    /// Set time to specific timestamp
    fn date_stamp(&self, sec: i64, ms: i64) -> Result<Self, ()> {
        Ok(Self { time: convert::date_stamp(sec, ms) })
    }

    /// Set time to specific year, month and day
    fn date_ymd(&self, year: i64, month: i64, day: i64) -> Result<Self, ()> {
        Ok(Self { time: convert::date_ymd(self.time, year, month, day)? })
    }

    /// Move time into previous or upcoming weekday
    fn offset_weekday(&self, new_weekday: i64, change: convert::Change) -> Result<Self, ()> {
        Ok(Self { time: convert::offset_weekday(self.time, new_weekday, change) })
    }

    /// Move time within month range
    fn offset_range_month(&self, target: TimeUnit, month: i64, change: convert::Change) -> Result<Self, ()> {
        if target.eq(&TimeUnit::Days) {
            let new_time = convert::offset_range_month(self.time, month, change)?;
            return Ok(Self { time: new_time });
        }

        Err(())
    }

    /// Move time within unit range
    fn offset_range_unit(&self, target: TimeUnit, unit: TimeUnit, change: convert::Change) -> Result<Self, ()> {
        if !(target.eq(&TimeUnit::Days) && unit.eq(&TimeUnit::Months)) {
            return Err(());
        }

        let new_day: u32 = match change.eq(&convert::Change::Last) {
            true => convert::into_month_day(self.time.year(), self.time.month(), 32),
            false => 1,
        };

        Ok(Self { time: self.time.with_day(new_day).unwrap() })
    }

    /// Move time exactly by specified number of units
    fn offset_unit_exact(&self, target: TimeUnit, amount: i64, _rules: &Rules) -> Result<FuzzyDate, ()> {
        let new_time = match target {
            TimeUnit::Seconds => self.time + Duration::seconds(amount),
            TimeUnit::Minutes => self.time + Duration::minutes(amount),
            TimeUnit::Hours => self.time + Duration::hours(amount),
            TimeUnit::Days => self.time + Duration::days(amount),
            TimeUnit::Weeks => self.time + Duration::days(amount * 7),
            TimeUnit::Months => convert::offset_months(self.time, amount),
            TimeUnit::Years => convert::offset_years(self.time, amount),
            _ => self.time,
        };

        Ok(Self { time: new_time })
    }

    /// Move time by specific unit, but apply keyword rules where
    /// e.g. moving by weeks will land on to first day of week
    fn offset_unit_keyword(&self, target: TimeUnit, amount: i64, rules: &Rules) -> Result<FuzzyDate, ()> {
        let new_time = match target {
            TimeUnit::Weeks => convert::offset_weeks(self.time, amount, rules.week_start_day()),
            _ => return self.offset_unit_exact(target, amount, rules)
        };

        Ok(Self { time: new_time })
    }

    /// Set time to specific hour, minute and second
    fn time_hms(&self, hour: i64, min: i64, sec: i64) -> Result<Self, ()> {
        Ok(Self { time: convert::time_hms(self.time, hour, min, sec)? })
    }

    /// Reset time to midnight
    fn time_reset(&self) -> Result<Self, ()> {
        self.time_hms(0, 0, 0)
    }

    /// Current year
    fn year(&self) -> i64 {
        self.time.year() as i64
    }
}


struct Rules {
    week_start_mon: bool,
}

impl Rules {
    fn week_start_day(&self) -> i8 {
        match self.week_start_mon {
            true => 1,
            false => 7,
        }
    }
}

/// Perform conversion against pattern and corresponding token values,
/// relative to given datetime
pub(crate) fn convert(
    pattern: &str,
    values: &Vec<i64>,
    current_time: &DateTime<FixedOffset>,
    week_start_mon: bool,
    custom_patterns: HashMap<String, String>) -> Option<DateTime<FixedOffset>> {
    let call_list = find_pattern_calls(&pattern, custom_patterns);

    if call_list.len().eq(&0) {
        return None;
    }

    let rules = Rules { week_start_mon: week_start_mon };
    let mut ctx_time = FuzzyDate::new(current_time.to_owned());
    let mut values: Vec<i64> = values.to_owned();

    for (pattern_match, pattern_call) in call_list {
        ctx_time = match pattern_call(ctx_time, &values, &rules) {
            Ok(value) => value,
            Err(()) => return None,
        };
        let used_vars: usize = pattern_match.split("[").count() - 1;
        values = values[used_vars..].to_owned();
    }

    Option::from(ctx_time.time)
}

/// Find closure calls that match the pattern exactly, or partially
fn find_pattern_calls(
    pattern: &str,
    custom: HashMap<String, String>) -> Vec<(String, fn(FuzzyDate, &Vec<i64>, &Rules) -> Result<FuzzyDate, ()>)> {
    let mut closure_map = HashMap::new();
    let mut pattern_map = HashMap::new();

    for (pattern_type, closure_function) in FUZZY_PATTERNS {
        closure_map.insert(pattern_type, closure_function);
        pattern_map.insert(Pattern::value(pattern_type).to_string(), pattern_type.to_owned());
    }

    for (custom_pattern, closure_pattern) in custom {
        if pattern_map.contains_key(&closure_pattern) {
            pattern_map.insert(custom_pattern.to_owned(), pattern_map.get(&closure_pattern).unwrap());
        }
    }

    for prefix in vec!["", "+"] {
        let try_pattern = format!("{}{}", prefix, pattern);

        if pattern_map.contains_key(&try_pattern) {
            let pattern_type = pattern_map.get(&try_pattern).unwrap();
            return vec![(try_pattern.to_string(), *closure_map.get(pattern_type).unwrap())];
        }
    }

    let mut result = vec![];
    let mut search = pattern;
    let prefix = if pattern.starts_with("-") { "-" } else { "+" };

    while search.len().gt(&0) {
        let mut calls: Vec<(&str, &Pattern)> = vec![];

        for (map_pattern, map_type) in &pattern_map {
            if search.starts_with(&map_pattern.as_str())
                || format!("{}{}", prefix, search).starts_with(&map_pattern.as_str()) {
                calls.push((&map_pattern, map_type));
            }
        }

        if calls.len().eq(&0) {
            return vec![];
        }

        calls.sort_by(|a, b| b.0.cmp(a.0));
        let (best_match, best_type) = calls.first().unwrap();

        search = &search[cmp::min(best_match.len(), search.len())..].trim_start();
        result.push((best_match.to_string(), *closure_map.get(best_type).unwrap()));
    }

    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_patterns() {
        let custom_finnish = vec![
            ("viime [wday]", &Pattern::LastWday),
            ("edellinen [wday]", &Pattern::PrevWday),
            ("ensi [wday]", &Pattern::NextWday),
            ("seuraava [wday]", &Pattern::NextWday),
        ];

        let result_value = convert_custom(
            "viime [wday]", vec![1],
            "2024-01-19T15:22:28+02:00", &custom_finnish,
        );
        assert_eq!(result_value, "2024-01-15 00:00:00 +02:00");

        let result_value = convert_custom(
            "edellinen [wday]", vec![1],
            "2024-01-19T15:22:28+02:00", &custom_finnish,
        );
        assert_eq!(result_value, "2024-01-15 00:00:00 +02:00");

        let result_value = convert_custom(
            "ensi [wday]", vec![1],
            "2024-01-19T15:22:28+02:00", &custom_finnish,
        );
        assert_eq!(result_value, "2024-01-22 00:00:00 +02:00");

        let result_value = convert_custom(
            "seuraava [wday]", vec![1],
            "2024-01-19T15:22:28+02:00", &custom_finnish,
        );
        assert_eq!(result_value, "2024-01-22 00:00:00 +02:00");
    }

    fn convert_custom(
        pattern: &str,
        values: Vec<i64>,
        current_time: &str,
        custom: &Vec<(&str, &Pattern)>) -> String {
        let current_time = DateTime::parse_from_rfc3339(current_time).unwrap();
        let mut custom_patterns: HashMap<String, String> = HashMap::new();

        for (key, value) in custom {
            custom_patterns.insert(key.to_string(), Pattern::value(value).to_string());
        }

        let result_time = convert(
            pattern,
            &values,
            &current_time,
            false,
            custom_patterns,
        );
        result_time.unwrap().to_string()
    }
}
