use crate::constants::Pattern;
use crate::convert;
use crate::token::Token;
use chrono::{DateTime, Datelike, Duration, FixedOffset};
use std::cmp;
use std::cmp::{Ordering, PartialEq};
use std::collections::HashMap;

const FUZZY_PATTERNS: [(&Pattern, fn(FuzzyDate, &CallValues, &Rules) -> Result<FuzzyDate, ()>); 55] = [
    // KEYWORDS
    (&Pattern::Now, |c, _, _| Ok(c)),
    (&Pattern::Today, |c, _, _| c.time_reset()),
    (&Pattern::Midnight, |c, _, _| c.time_reset()),
    (&Pattern::Yesterday, |c, _, r| c.offset_unit_keyword(TimeUnit::Days, -1, r)?.time_reset()),
    (&Pattern::Tomorrow, |c, _, r| c.offset_unit_keyword(TimeUnit::Days, 1, r)?.time_reset()),
    // WEEKDAY OFFSETS
    (&Pattern::ThisWday, |c, v, _| c.offset_weekday(v.get_int(0), convert::Change::None)?.time_reset()),
    (&Pattern::PrevWday, |c, v, _| c.offset_weekday(v.get_int(0), convert::Change::Prev)?.time_reset()),
    (&Pattern::NextWday, |c, v, _| c.offset_weekday(v.get_int(0), convert::Change::Next)?.time_reset()),
    // MONTH OFFSETS
    (&Pattern::ThisMonth, |c, v, _| c.offset_month(v.get_int(0), convert::Change::None)?.time_reset()),
    (&Pattern::PrevMonth, |c, v, _| c.offset_month(v.get_int(0), convert::Change::Prev)?.time_reset()),
    (&Pattern::NextMonth, |c, v, _| c.offset_month(v.get_int(0), convert::Change::Next)?.time_reset()),
    // KEYWORD OFFSETS
    (&Pattern::ThisLongUnit, |c, v, r| c.offset_unit_keyword(v.get_unit(0), 0, r)),
    (&Pattern::PastLongUnit, |c, v, r| c.offset_unit_exact(v.get_unit(0), -1, r)),
    (&Pattern::PrevLongUnit, |c, v, r| c.offset_unit_keyword(v.get_unit(0), -1, r)),
    (&Pattern::PrevNLongUnit, |c, v, r| c.offset_unit_keyword(v.get_unit(1), 0 - v.get_int(0), r)),
    (&Pattern::NextLongUnit, |c, v, r| c.offset_unit_keyword(v.get_unit(0), 1, r)),
    // NUMERIC OFFSET, MINUS
    (&Pattern::MinusUnit, |c, v, r| c.offset_unit_exact(v.get_unit(1), 0 - v.get_int(0), r)),
    (&Pattern::MinusShortUnit, |c, v, r| c.offset_unit_exact(v.get_unit(1), 0 - v.get_int(0), r)),
    (&Pattern::MinusLongUnit, |c, v, r| c.offset_unit_exact(v.get_unit(1), 0 - v.get_int(0), r)),
    // NUMERIC OFFSET, PLUS
    (&Pattern::PlusUnit, |c, v, r| c.offset_unit_exact(v.get_unit(1), v.get_int(0), r)),
    (&Pattern::PlusShortUnit, |c, v, r| c.offset_unit_exact(v.get_unit(1), v.get_int(0), r)),
    (&Pattern::PlusLongUnit, |c, v, r| c.offset_unit_exact(v.get_unit(1), v.get_int(0), r)),
    // NUMERIC OFFSET, PLUS
    (&Pattern::UnitAgo, |c, v, r| c.offset_unit_exact(v.get_unit(1), 0 - v.get_int(0), r)),
    (&Pattern::LongUnitAgo, |c, v, r| c.offset_unit_exact(v.get_unit(1), 0 - v.get_int(0), r)),
    // FIRST/LAST OFFSETS
    (&Pattern::FirstLongUnitOfMonth, |c, v, _| {
        c.offset_range_month(v.get_unit(0), v.get_int(1), convert::Change::First)?
            .time_reset()
    }),
    (&Pattern::FirstLongUnitOfMonthYear, |c, v, _| {
        c.offset_range_year_month(v.get_unit(0), v.get_int(2), v.get_int(1), convert::Change::First)?
            .time_reset()
    }),
    (&Pattern::FirstLongUnitOfYear, |c, v, _| {
        c.offset_range_year_month(v.get_unit(0), v.get_int(1), 1, convert::Change::First)?
            .time_reset()
    }),
    (&Pattern::LastLongUnitOfMonth, |c, v, _| {
        c.offset_range_month(v.get_unit(0), v.get_int(1), convert::Change::Last)?
            .time_reset()
    }),
    (&Pattern::LastLongUnitOfMonthYear, |c, v, _| {
        c.offset_range_year_month(v.get_unit(0), v.get_int(2), v.get_int(1), convert::Change::Last)?
            .time_reset()
    }),
    (&Pattern::LastLongUnitOfYear, |c, v, _| {
        c.offset_range_year_month(v.get_unit(0), v.get_int(1), 12, convert::Change::Last)?
            .time_reset()
    }),
    (&Pattern::FirstLongUnitOfThisLongUnit, |c, v, _| {
        c.offset_range_unit(v.get_unit(0), v.get_unit(1), convert::Change::First)?
            .time_reset()
    }),
    (&Pattern::LastLongUnitOfThisLongUnit, |c, v, _| {
        c.offset_range_unit(v.get_unit(0), v.get_unit(1), convert::Change::Last)?
            .time_reset()
    }),
    (&Pattern::FirstLongUnitOfPrevLongUnit, |c, v, r| {
        c.offset_unit_keyword(v.get_unit(1), -1, r)?
            .offset_range_unit(v.get_unit(0), v.get_unit(1), convert::Change::First)?
            .time_reset()
    }),
    (&Pattern::LastLongUnitOfPrevLongUnit, |c, v, r| {
        c.offset_unit_keyword(v.get_unit(1), -1, r)?
            .offset_range_unit(v.get_unit(0), v.get_unit(1), convert::Change::Last)?
            .time_reset()
    }),
    (&Pattern::FirstLongUnitOfNextLongUnit, |c, v, r| {
        c.offset_unit_keyword(v.get_unit(1), 1, r)?
            .offset_range_unit(v.get_unit(0), v.get_unit(1), convert::Change::First)?
            .time_reset()
    }),
    (&Pattern::LastLongUnitOfNextLongUnit, |c, v, r| {
        c.offset_unit_keyword(v.get_unit(1), 1, r)?
            .offset_range_unit(v.get_unit(0), v.get_unit(1), convert::Change::Last)?
            .time_reset()
    }),
    // 20230130
    (&Pattern::Integer, |c, v, _| c.date_iso8601(v.get_string(0))?.time_reset()),
    // April, April 2023
    (&Pattern::Month, |c, v, _| c.date_ym(c.year(), v.get_int(0))?.time_reset()),
    (&Pattern::MonthYear, |c, v, _| c.date_ymd(v.get_int(1), v.get_int(0), 1)?.time_reset()),
    // @1705072948, @1705072948.452
    (&Pattern::Timestamp, |c, v, _| c.date_stamp(v.get_int(0), 0)),
    (&Pattern::TimestampFloat, |c, v, _| c.date_stamp(v.get_int(0), v.get_ms(1))),
    // 2023-01-30, 30.1.2023, 1/30/2023
    (&Pattern::DateYmd, |c, v, _| c.date_ymd(v.get_int(0), v.get_int(1), v.get_int(2))?.time_reset()),
    (&Pattern::DateDmy, |c, v, _| c.date_ymd(v.get_int(2), v.get_int(1), v.get_int(0))?.time_reset()),
    (&Pattern::DateMdy, |c, v, _| c.date_ymd(v.get_int(2), v.get_int(0), v.get_int(1))?.time_reset()),
    // Dec 7, Dec 7th, 7 Dec
    (&Pattern::DateMonthDay, |c, v, _| c.date_ymd(c.year(), v.get_int(0), v.get_int(1))?.time_reset()),
    (&Pattern::DateMonthNth, |c, v, _| c.date_ymd(c.year(), v.get_int(0), v.get_int(1))?.time_reset()),
    (&Pattern::DateDayMonth, |c, v, _| c.date_ymd(c.year(), v.get_int(1), v.get_int(0))?.time_reset()),
    // Dec 7 2023, Dec 7th 2023, 7 Dec 2023
    (&Pattern::DateMonthDayYear, |c, v, _| c.date_ymd(v.get_int(2), v.get_int(0), v.get_int(1))?.time_reset()),
    (&Pattern::DateMonthNthYear, |c, v, _| c.date_ymd(v.get_int(2), v.get_int(0), v.get_int(1))?.time_reset()),
    (&Pattern::DateDayMonthYear, |c, v, _| c.date_ymd(v.get_int(2), v.get_int(1), v.get_int(0))?.time_reset()),
    // 2023-12-07 15:02, 2023-12-07 15:02:01, 2023-12-07 15:02:01.456
    (&Pattern::DateTimeYmdHm, |c, v, _| {
        c.date_ymd(v.get_int(0), v.get_int(1), v.get_int(2))?
            .time_hms(v.get_int(3), v.get_int(4), 0, 0)
    }),
    (&Pattern::DateTimeYmdHms, |c, v, _| {
        c.date_ymd(v.get_int(0), v.get_int(1), v.get_int(2))?
            .time_hms(v.get_int(3), v.get_int(4), v.get_int(5), 0)
    }),
    (&Pattern::DateTimeYmdHmsMs, |c, v, _| {
        c.date_ymd(v.get_int(0), v.get_int(1), v.get_int(2))?.time_hms(
            v.get_int(3),
            v.get_int(4),
            v.get_int(5),
            v.get_ms(6),
        )
    }),
    // 3pm, 3:00 pm
    (&Pattern::TimeMeridiemH, |c, v, _| c.time_12h(v.get_int(0), 0, 0, v.get_int(1))),
    (&Pattern::TimeMeridiemHm, |c, v, _| c.time_12h(v.get_int(0), v.get_int(1), 0, v.get_int(2))),
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

struct CallValues {
    tokens: Vec<Token>,
}

impl CallValues {
    fn from_tokens(tokens: Vec<Token>) -> Self {
        Self { tokens: tokens }
    }

    fn drop_used(&mut self, used: usize) {
        self.tokens = self.tokens[used..].to_owned();
    }

    fn get_int(&self, index: usize) -> i64 {
        self.tokens[index].value
    }

    fn get_string(&self, index: usize) -> String {
        let value = self.tokens[index].value;
        let zeros = self.tokens[index].zeros;
        format!("{}{}", "0".repeat(zeros as usize), value)
    }

    /// Get value with the assumption that it should represent milliseconds,
    /// and thus the number of zeros before the number is meaningful. If there
    /// are too many zeros, we use -1 to break out on millisecond value
    /// validation.
    fn get_ms(&self, index: usize) -> i64 {
        let value = self.tokens[index].value;
        let zeros = self.tokens[index].zeros;

        let multiply_by = if value.lt(&10) {
            match zeros {
                0 => 100,
                1 => 10,
                2 => 1,
                _ => return -1,
            }
        } else if value.lt(&100) {
            match zeros {
                0 => 10,
                1 => 1,
                _ => return -1,
            }
        } else if value.lt(&1000) {
            match zeros {
                0 => 1,
                _ => return -1,
            }
        } else {
            return -1;
        };

        value * multiply_by
    }

    fn get_unit(&self, index: usize) -> TimeUnit {
        TimeUnit::from_int(self.get_int(index))
    }
}

struct FuzzyDate {
    time: DateTime<FixedOffset>,
}

impl FuzzyDate {
    fn new(time: DateTime<FixedOffset>) -> Self {
        FuzzyDate { time: time }
    }

    /// Set time to specific data from basic ISO8601 date string
    fn date_iso8601(&self, value: String) -> Result<Self, ()> {
        Ok(Self { time: convert::date_iso8601(self.time, value)? })
    }

    /// Set time to specific timestamp
    fn date_stamp(&self, sec: i64, ms: i64) -> Result<Self, ()> {
        Ok(Self { time: convert::date_stamp(sec, ms) })
    }

    /// Set time to specific year and month
    fn date_ym(&self, year: i64, month: i64) -> Result<Self, ()> {
        let month_day = convert::into_month_day(year as i32, month as u32, self.time.day());
        Ok(Self { time: convert::date_ymd(self.time, year, month, month_day as i64)? })
    }

    /// Set time to specific year, month and day
    fn date_ymd(&self, year: i64, month: i64, day: i64) -> Result<Self, ()> {
        Ok(Self { time: convert::date_ymd(self.time, year, month, day)? })
    }

    /// Move time into previous or upcoming month
    fn offset_month(&self, new_month: i64, change: convert::Change) -> Result<Self, ()> {
        Ok(Self { time: convert::offset_month(self.time, new_month, change) })
    }

    /// Move time into previous or upcoming weekday
    fn offset_weekday(&self, new_weekday: i64, change: convert::Change) -> Result<Self, ()> {
        Ok(Self { time: convert::offset_weekday(self.time, new_weekday, change) })
    }

    /// Move time within month range
    fn offset_range_month(&self, target: TimeUnit, month: i64, change: convert::Change) -> Result<Self, ()> {
        if target.eq(&TimeUnit::Days) {
            let new_time = convert::offset_range_year_month(self.time, self.time.year() as i64, month, change)?;
            return Ok(Self { time: new_time });
        }

        Err(())
    }

    /// Move time within unit range
    fn offset_range_unit(&self, target: TimeUnit, unit: TimeUnit, change: convert::Change) -> Result<Self, ()> {
        if target.eq(&TimeUnit::Days) && unit.eq(&TimeUnit::Years) {
            if change.eq(&convert::Change::Last) {
                let last_day = convert::into_month_day(self.time.year(), 12, 31);
                return self.date_ymd(self.time.year() as i64, 12, last_day as i64);
            }

            return self.date_ymd(self.time.year() as i64, 1, 1);
        }

        if target.eq(&TimeUnit::Days) && unit.eq(&TimeUnit::Months) {
            if change.eq(&convert::Change::Last) {
                let last_day = convert::into_month_day(self.time.year(), self.time.month(), 31);
                return Ok(Self { time: self.time.with_day(last_day).unwrap() });
            }

            return Ok(Self { time: self.time.with_day(1).unwrap() });
        }

        Err(())
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
            _ => return self.offset_unit_exact(target, amount, rules),
        };

        Ok(Self { time: new_time })
    }

    /// Move time within year and month range
    fn offset_range_year_month(
        &self,
        target: TimeUnit,
        year: i64,
        month: i64,
        change: convert::Change,
    ) -> Result<Self, ()> {
        if target.eq(&TimeUnit::Days) {
            let new_time = convert::offset_range_year_month(self.time, year, month, change)?;
            return Ok(Self { time: new_time });
        }

        Err(())
    }

    /// Set time to specific hour, minute and second using 12-hour clock
    fn time_12h(&self, hour: i64, min: i64, sec: i64, meridiem: i64) -> Result<Self, ()> {
        Ok(Self { time: convert::time_12h(self.time, hour, min, sec, meridiem)? })
    }

    /// Set time to specific hour, minute and second
    fn time_hms(&self, hour: i64, min: i64, sec: i64, ms: i64) -> Result<Self, ()> {
        Ok(Self { time: convert::time_hms(self.time, hour, min, sec, ms)? })
    }

    /// Reset time to midnight
    fn time_reset(&self) -> Result<Self, ()> {
        self.time_hms(0, 0, 0, 0)
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

pub(crate) struct UnitLocale {
    day: String,
    days: String,
    hour: String,
    hours: String,
    minute: String,
    minutes: String,
    second: String,
    seconds: String,
    week: String,
    weeks: String,
    separator: String,
}

impl UnitLocale {
    pub(crate) fn from_map(names: HashMap<String, String>) -> Self {
        let mut mapping: HashMap<String, String> = HashMap::from([
            (String::from("second"), String::new()),
            (String::from("seconds"), String::new()),
            (String::from("minute"), String::new()),
            (String::from("minutes"), String::new()),
            (String::from("hour"), String::new()),
            (String::from("hours"), String::new()),
            (String::from("day"), String::new()),
            (String::from("days"), String::new()),
            (String::from("week"), String::new()),
            (String::from("weeks"), String::new()),
        ]);

        mapping.extend(names);

        Self {
            day: mapping.get("day").unwrap().to_owned(),
            days: mapping.get("days").unwrap().to_owned(),
            hour: mapping.get("hour").unwrap().to_owned(),
            hours: mapping.get("hours").unwrap().to_owned(),
            minute: mapping.get("minute").unwrap().to_owned(),
            minutes: mapping.get("minutes").unwrap().to_owned(),
            second: mapping.get("second").unwrap().to_owned(),
            seconds: mapping.get("seconds").unwrap().to_owned(),
            week: mapping.get("week").unwrap().to_owned(),
            weeks: mapping.get("weeks").unwrap().to_owned(),
            separator: if mapping.get("day").unwrap().len() > 1 { " " } else { "" }.to_owned(),
        }
    }

    fn format_days(&self, amount: i32) -> String {
        let unit = if amount.eq(&1) { &self.day } else { &self.days };
        format!(" {}{}{}", amount, self.separator, unit)
    }

    fn format_hours(&self, amount: i32) -> String {
        let unit = if amount.eq(&1) { &self.hour } else { &self.hours };
        format!(" {}{}{}", amount, self.separator, unit)
    }

    fn format_minutes(&self, amount: i32) -> String {
        let unit = if amount.eq(&1) { &self.minute } else { &self.minutes };
        format!(" {}{}{}", amount, self.separator, unit)
    }

    fn format_seconds(&self, amount: i32) -> String {
        let unit = if amount.eq(&1) { &self.second } else { &self.seconds };
        format!(" {}{}{}", amount, self.separator, unit)
    }

    fn format_weeks(&self, amount: i32) -> String {
        let unit = if amount.eq(&1) { &self.week } else { &self.weeks };
        format!(" {}{}{}", amount, self.separator, unit)
    }
}

/// Perform conversion against pattern and corresponding token values,
/// relative to given datetime
pub(crate) fn convert(
    pattern: &str,
    tokens: Vec<Token>,
    current_time: &DateTime<FixedOffset>,
    week_start_mon: bool,
    custom_patterns: HashMap<String, String>,
) -> Option<DateTime<FixedOffset>> {
    let call_list = find_pattern_calls(&pattern, custom_patterns);

    if call_list.is_empty() {
        return None;
    }

    let rules = Rules { week_start_mon: week_start_mon };

    let mut ctx_time = FuzzyDate::new(current_time.to_owned());
    let mut ctx_vals = CallValues::from_tokens(tokens);

    for (pattern_match, pattern_call) in call_list {
        ctx_time = match pattern_call(ctx_time, &ctx_vals, &rules) {
            Ok(value) => value,
            Err(_) => return None,
        };
        ctx_vals.drop_used(pattern_match.split("[").count() - 1);
    }

    Some(ctx_time.time)
}

/// Turn seconds into a duration string
pub(crate) fn to_duration(seconds: f64, units: &UnitLocale, max_unit: &str, min_unit: &str) -> String {
    let mut seconds = seconds;
    let mut result: String = String::new();

    let naming: HashMap<&str, i8> = HashMap::from([
        ("s", 1),
        ("sec", 1),
        ("min", 2),
        ("mins", 2),
        ("h", 3),
        ("hr", 3),
        ("hrs", 3),
        ("d", 4),
        ("day", 4),
        ("days", 4),
        ("w", 5),
        ("week", 5),
        ("weeks", 5),
    ]);

    let max_u: &i8 = naming.get(max_unit).unwrap_or(&5);
    let min_u: &i8 = naming.get(min_unit).unwrap_or(&1);

    if max_u.ge(&5) && min_u.le(&5) {
        let weeks = (seconds / 604800.0).floor() as i32;

        if weeks.gt(&0) {
            result.push_str(&units.format_weeks(weeks));
            seconds -= (weeks * 604800) as f64;
        }
    }

    if max_u.ge(&4) && min_u.le(&4) {
        let days = (seconds / 86400.0).floor() as i32;

        if days.gt(&0) {
            result.push_str(&units.format_days(days));
            seconds -= (days * 86400) as f64;
        }
    }

    if max_u.ge(&3) && min_u.le(&3) {
        let hours = (seconds / 3600.0).floor() as i32;

        if hours.gt(&0) {
            result.push_str(&units.format_hours(hours));
            seconds -= (hours * 3600) as f64;
        }
    }

    if max_u.ge(&2) && min_u.le(&2) {
        let minutes = (seconds / 60.0).floor() as i32;

        if minutes.gt(&0) {
            result.push_str(&units.format_minutes(minutes));
            seconds -= (minutes * 60) as f64;
        }
    }

    if max_u.ge(&1) && min_u.le(&1) {
        if seconds.gt(&0.0) {
            result.push_str(&units.format_seconds(seconds as i32));
        }
    }

    result.trim().to_string()
}

/// Find closure calls that match the pattern exactly, or partially
fn find_pattern_calls(
    pattern: &str,
    custom: HashMap<String, String>,
) -> Vec<(String, fn(FuzzyDate, &CallValues, &Rules) -> Result<FuzzyDate, ()>)> {
    let mut closure_map = HashMap::new();
    let mut pattern_map = HashMap::new();

    for (pattern_type, closure_function) in FUZZY_PATTERNS {
        closure_map.insert(pattern_type, closure_function);
        for pattern_value in Pattern::values(pattern_type) {
            pattern_map.insert(pattern_value.to_string(), pattern_type.to_owned());
        }
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

    let mut result = Vec::new();
    let mut search = pattern;
    let prefix = if pattern.starts_with("-") { "-" } else { "+" };

    while !search.is_empty() {
        let mut calls: Vec<(&str, &Pattern)> = Vec::new();
        let searches = Vec::from([search.to_string(), format!("{}{}", prefix, search)]);

        for (map_pattern, map_type) in &pattern_map {
            if is_pattern_match(&searches, &map_pattern) {
                calls.push((&map_pattern, map_type));
            }
        }

        if calls.is_empty() {
            return Vec::new();
        }

        if calls.len().gt(&1) {
            calls.sort_by(|a, b| match b.0.len().cmp(&a.0.len()) {
                Ordering::Equal => a.0.cmp(b.0),
                v => v,
            });
        }

        let (best_match, best_type) = calls.first().unwrap();

        search = &search[cmp::min(best_match.len(), search.len())..].trim_start();
        result.push((best_match.to_string(), *closure_map.get(best_type).unwrap()));
    }

    result
}

/// Check if the pattern string matches to any of the given strings
fn is_pattern_match(searches: &Vec<String>, pattern: &String) -> bool {
    if searches.contains(&pattern) {
        return true;
    }

    for search in searches {
        if !search.starts_with(pattern) {
            continue;
        }

        // Next character in the source string must be a space, to prevent matches
        // that have overlapping parts to match incorrectly.
        //
        // For example "[month] [int][meridiem]" could otherwise first match to
        // "[month] [int]" rather than to "[month]" and then to "[int][meridiem]".
        //
        // We use a space to identify them as fully separate subpattern matches.
        if search[pattern.len()..pattern.len() + 1].eq(" ") {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_patterns() {
        let custom_finnish = vec![
            ("viime [wday]", &Pattern::PrevWday),
            ("edellinen [wday]", &Pattern::PrevWday),
            ("ensi [wday]", &Pattern::NextWday),
            ("seuraava [wday]", &Pattern::NextWday),
        ];

        let result_value = convert_custom("viime [wday]", vec![1], "2024-01-19T15:22:28+02:00", &custom_finnish);
        assert_eq!(result_value, "2024-01-15 00:00:00 +02:00");

        let result_value = convert_custom("edellinen [wday]", vec![1], "2024-01-19T15:22:28+02:00", &custom_finnish);
        assert_eq!(result_value, "2024-01-15 00:00:00 +02:00");

        let result_value = convert_custom("ensi [wday]", vec![1], "2024-01-19T15:22:28+02:00", &custom_finnish);
        assert_eq!(result_value, "2024-01-22 00:00:00 +02:00");

        let result_value = convert_custom("seuraava [wday]", vec![1], "2024-01-19T15:22:28+02:00", &custom_finnish);
        assert_eq!(result_value, "2024-01-22 00:00:00 +02:00");
    }

    fn convert_custom(pattern: &str, values: Vec<i64>, current_time: &str, custom: &Vec<(&str, &Pattern)>) -> String {
        let current_time = DateTime::parse_from_rfc3339(current_time).unwrap();
        let mut custom_patterns: HashMap<String, String> = HashMap::new();

        for (key, value) in custom {
            for pattern_value in Pattern::values(value) {
                custom_patterns.insert(key.to_string(), pattern_value.to_string());
            }
        }

        let tokens = values
            .iter()
            .map(|v| Token::new_integer(v.to_owned(), 0))
            .collect::<Vec<Token>>();

        let result_time = convert(pattern, tokens, &current_time, false, custom_patterns);
        result_time.unwrap().to_string()
    }
}
