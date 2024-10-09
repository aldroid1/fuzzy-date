use chrono::{DateTime, Datelike, Duration, FixedOffset};
use std::{cmp};
use std::cmp::PartialEq;
use std::collections::HashMap;
use crate::convert;

const FUZZY_PATTERNS: [(&'static str, fn(FuzzyDate, &Vec<i64>, &Rules) -> Result<FuzzyDate, ()>); 41] = [
    // KEYWORDS
    ("now", |c, _, _| Ok(c)),
    ("today", |c, _, _| c.time_reset()),
    ("midnight", |c, _, _| c.time_reset()),
    ("yesterday", |c, _, r| c.offset_unit(TimeUnit::Days, -1, r)),
    ("tomorrow", |c, _, r| c.offset_unit(TimeUnit::Days, 1, r)),

    // KEYWORD OFFSETS
    ("this [long_unit]", |c, v, r| c.offset_unit(TimeUnit::from_int(v[0]), 0, r)),
    ("this [wday]", |c, v, _| c.offset_weekday(v[0], convert::Change::None)),
    ("last [long_unit]", |c, v, r| c.offset_unit(TimeUnit::from_int(v[0]), -1, r)),
    ("last [wday]", |c, v, _| c.offset_weekday(v[0], convert::Change::Prev)),
    ("prev [long_unit]", |c, v, r| c.offset_unit(TimeUnit::from_int(v[0]), -1, r)),
    ("prev [wday]", |c, v, _| c.offset_weekday(v[0], convert::Change::Prev)),
    ("next [long_unit]", |c, v, r| c.offset_unit(TimeUnit::from_int(v[0]), 1, r)),
    ("next [wday]", |c, v, _| c.offset_weekday(v[0], convert::Change::Next)),

    // NUMERIC OFFSET
    ("-[int][unit]", |c, v, r| c.offset_unit(TimeUnit::from_int(v[1]), 0 - v[0], r)),
    ("-[int][short_unit]", |c, v, r| c.offset_unit(TimeUnit::from_int(v[1]), 0 - v[0], r)),
    ("-[int] [long_unit]", |c, v, r| c.offset_unit(TimeUnit::from_int(v[1]), 0 - v[0], r)),
    ("+[int][unit]", |c, v, r| c.offset_unit(TimeUnit::from_int(v[1]), v[0], r)),
    ("+[int][short_unit]", |c, v, r| c.offset_unit(TimeUnit::from_int(v[1]), v[0], r)),
    ("+[int] [long_unit]", |c, v, r| c.offset_unit(TimeUnit::from_int(v[1]), v[0], r)),
    ("[int] [unit] ago", |c, v, r| c.offset_unit(TimeUnit::from_int(v[1]), 0 - v[0], r)),
    ("[int] [long_unit] ago", |c, v, r| c.offset_unit(TimeUnit::from_int(v[1]), 0 - v[0], r)),

    // FIRST/LAST OFFSETS
    ("first [long_unit] of [month]", |c, v, _| c
        .offset_range_month(TimeUnit::from_int(v[0]), v[1], convert::Change::First)?
        .time_reset(),
    ),
    ("last [long_unit] of [month]", |c, v, _| c
        .offset_range_month(TimeUnit::from_int(v[0]), v[1], convert::Change::Last)?
        .time_reset(),
    ),
    ("first [long_unit] of this [long_unit]", |c, v, _| c
        .offset_range_unit(TimeUnit::from_int(v[0]), TimeUnit::from_int(v[1]), convert::Change::First)?
        .time_reset(),
    ),
    ("last [long_unit] of this [long_unit]", |c, v, _| c
        .offset_range_unit(TimeUnit::from_int(v[0]), TimeUnit::from_int(v[1]), convert::Change::Last)?
        .time_reset(),
    ),
    ("first [long_unit] of prev [long_unit]", |c, v, r| c
        .offset_unit(TimeUnit::from_int(v[1]), -1, r)?
        .offset_range_unit(TimeUnit::from_int(v[0]), TimeUnit::from_int(v[1]), convert::Change::First)?
        .time_reset(),
    ),
    ("last [long_unit] of prev [long_unit]", |c, v, r| c
        .offset_unit(TimeUnit::from_int(v[1]), -1, r)?
        .offset_range_unit(TimeUnit::from_int(v[0]), TimeUnit::from_int(v[1]), convert::Change::Last)?
        .time_reset(),
    ),
    ("first [long_unit] of last [long_unit]", |c, v, r| c
        .offset_unit(TimeUnit::from_int(v[1]), -1, r)?
        .offset_range_unit(TimeUnit::from_int(v[0]), TimeUnit::from_int(v[1]), convert::Change::First)?
        .time_reset(),
    ),
    ("last [long_unit] of last [long_unit]", |c, v, r| c
        .offset_unit(TimeUnit::from_int(v[1]), -1, r)?
        .offset_range_unit(TimeUnit::from_int(v[0]), TimeUnit::from_int(v[1]), convert::Change::Last)?
        .time_reset(),
    ),
    ("first [long_unit] of next [long_unit]", |c, v, r| c
        .offset_unit(TimeUnit::from_int(v[1]), 1, r)?
        .offset_range_unit(TimeUnit::from_int(v[0]), TimeUnit::from_int(v[1]), convert::Change::First)?
        .time_reset(),
    ),
    ("last [long_unit] of next [long_unit]", |c, v, r| c
        .offset_unit(TimeUnit::from_int(v[1]), 1, r)?
        .offset_range_unit(TimeUnit::from_int(v[0]), TimeUnit::from_int(v[1]), convert::Change::Last)?
        .time_reset(),
    ),

    // @1705072948, @1705072948.452
    ("[timestamp]", |c, v, _| c.date_stamp(v[0], 0)),
    ("[timestamp].[int]", |c, v, _| c.date_stamp(v[0], v[1])),

    // 2023-01-01, 30.1.2023, 1/30/2023
    ("[year]-[int]-[int]", |c, v, _| c.date_ymd(v[0], v[1], v[2])?.time_reset()),
    ("[int].[int].[year]", |c, v, _| c.date_ymd(v[2], v[1], v[0])?.time_reset()),
    ("[int]/[int]/[year]", |c, v, _| c.date_ymd(v[2], v[0], v[1])?.time_reset()),

    // Dec 7 2023, Dec 7th 2023, 7 Dec 2023
    ("[month] [int] [year]", |c, v, _| c.date_ymd(v[2], v[0], v[1])?.time_reset()),
    ("[month] [nth] [year]", |c, v, _| c.date_ymd(v[2], v[0], v[1])?.time_reset()),
    ("[int] [month] [year]", |c, v, _| c.date_ymd(v[2], v[1], v[0])?.time_reset()),

    // 2023-12-07 15:02
    ("[year]-[int]-[int] [int]:[int]", |c, v, _| c
        .date_ymd(v[0], v[1], v[2])?.time_hms(v[3], v[4], 0)
    ),

    // 2023-12-07 15:02:01
    ("[year]-[int]-[int] [int]:[int]:[int]", |c, v, _| c
        .date_ymd(v[0], v[1], v[2])?.time_hms(v[3], v[4], v[5])
    ),
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

    /// Move time by specific unit
    fn offset_unit(&self, target: TimeUnit, amount: i64, rules: &Rules) -> Result<FuzzyDate, ()> {
        let new_time = match target {
            TimeUnit::Seconds => self.time + Duration::seconds(amount),
            TimeUnit::Minutes => self.time + Duration::minutes(amount),
            TimeUnit::Hours => self.time + Duration::hours(amount),
            TimeUnit::Days => self.time + Duration::days(amount),
            TimeUnit::Weeks => convert::offset_weeks(self.time, amount, rules.week_start_day()),
            TimeUnit::Months => convert::offset_months(self.time, amount),
            TimeUnit::Years => convert::offset_years(self.time, amount),
            _ => self.time,
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
pub(crate) fn convert(pattern: &str, values: &Vec<i64>, current_time: &DateTime<FixedOffset>, week_start_mon: bool) -> Option<DateTime<FixedOffset>> {
    let call_list = find_pattern_calls(&pattern);

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
fn find_pattern_calls(pattern: &str) -> Vec<(&str, fn(FuzzyDate, &Vec<i64>, &Rules) -> Result<FuzzyDate, ()>)> {
    let closure_map = HashMap::from(FUZZY_PATTERNS);

    if closure_map.contains_key(pattern) {
        return vec![(pattern, *closure_map.get(pattern).unwrap())];
    }

    let mut result = vec![];
    let mut search = pattern;
    let mut prefix = "+";

    if pattern.starts_with("-")
        || pattern.starts_with("prev")
        || pattern.starts_with("last") {
        prefix = "-";
    }

    while search.len().gt(&0) {
        let mut calls = vec![];

        for map_pattern in closure_map.keys() {
            if search.starts_with(map_pattern)
                || format!("{}{}", prefix, search).starts_with(map_pattern) {
                calls.push(*map_pattern);
            }
        }

        if calls.len().eq(&0) {
            return vec![];
        }

        calls.sort_by(|a, b| b.cmp(a));
        let best_match: &str = calls.first().unwrap();

        search = &search[cmp::min(best_match.len(), search.len())..].trim_start();
        result.push((best_match, *closure_map.get(*&best_match).unwrap()));
    }

    result
}


#[cfg(test)]
mod tests {}
