use crate::constants::Pattern;
use crate::convert;
use crate::token::Token;
use chrono::{DateTime, Datelike, Duration, FixedOffset};
use std::cmp;
use std::cmp::{Ordering, PartialEq};
use std::collections::{HashMap, HashSet};

const FUZZY_PATTERNS: [(&Pattern, fn(FuzzyDate, &CallValues, &Rules) -> Result<FuzzyDate, ()>); 74] = [
    // KEYWORDS
    (&Pattern::Now, |c, _, _| Ok(c)),
    (&Pattern::Today, |c, _, r| c.rule_time_reset(r)),
    (&Pattern::Midnight, |c, _, _| c.time_hms(0, 0, 0, 0)),
    (&Pattern::Yesterday, |c, _, r| c.offset_unit_keyword(TimeUnit::Days, -1, r)?.rule_time_reset(r)),
    (&Pattern::Tomorrow, |c, _, r| c.offset_unit_keyword(TimeUnit::Days, 1, r)?.rule_time_reset(r)),
    // WEEKDAY OFFSETS
    (&Pattern::Wday, |c, v, r| c.offset_current_weekday(v.get_int(0))?.rule_time_reset(r)),
    (&Pattern::ThisWday, |c, v, r| c.offset_weekday(v.get_int(0), convert::Change::None)?.rule_time_reset(r)),
    (&Pattern::PrevWday, |c, v, r| c.offset_weekday(v.get_int(0), convert::Change::Prev)?.rule_time_reset(r)),
    (&Pattern::NextWday, |c, v, r| c.offset_weekday(v.get_int(0), convert::Change::Next)?.rule_time_reset(r)),
    // MONTH OFFSETS
    (&Pattern::ThisMonth, |c, v, r| c.offset_month(v.get_int(0), convert::Change::None)?.rule_time_reset(r)),
    (&Pattern::PrevMonth, |c, v, r| c.offset_month(v.get_int(0), convert::Change::Prev)?.rule_time_reset(r)),
    (&Pattern::NextMonth, |c, v, r| c.offset_month(v.get_int(0), convert::Change::Next)?.rule_time_reset(r)),
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
    // EXACT UNIT
    (&Pattern::LongUnitInt, |c, v, r| {
        c.ensure_unit(v.get_unit(0), TimeUnit::Weeks)?
            .date_yw(c.rule_year(), v.get_int(1), r)?
            .rule_time_reset(r)
    }),
    (&Pattern::LongUnitIntYear, |c, v, r| {
        c.ensure_unit(v.get_unit(0), TimeUnit::Weeks)?
            .date_yw(v.get_int(2), v.get_int(1), r)?
            .rule_time_reset(r)
    }),
    // FIRST/LAST RELATIVE OFFSETS
    (&Pattern::FirstOfLongUnit, |c, v, r| {
        c.ensure_unit(v.get_unit(0), TimeUnit::Months)?
            .offset_range_month(TimeUnit::Days, c.month(), convert::Change::First)?
            .rule_time_reset(r)
    }),
    (&Pattern::FirstLongUnitOfMonth, |c, v, r| {
        c.offset_range_month(v.get_unit(0), v.get_int(1), convert::Change::First)?
            .rule_time_reset(r)
    }),
    (&Pattern::FirstLongUnitOfMonthYear, |c, v, r| {
        c.offset_range_year_month(v.get_unit(0), v.get_int(2), v.get_int(1), convert::Change::First)?
            .rule_time_reset(r)
    }),
    (&Pattern::FirstLongUnitOfYear, |c, v, r| {
        c.offset_range_year_month(v.get_unit(0), v.get_int(1), 1, convert::Change::First)?
            .rule_time_reset(r)
    }),
    (&Pattern::LastLongUnitOfMonth, |c, v, r| {
        c.offset_range_month(v.get_unit(0), v.get_int(1), convert::Change::Last)?
            .rule_time_reset(r)
    }),
    (&Pattern::LastOfLongUnit, |c, v, r| {
        c.ensure_unit(v.get_unit(0), TimeUnit::Months)?
            .offset_range_month(TimeUnit::Days, c.month(), convert::Change::Last)?
            .rule_time_reset(r)
    }),
    (&Pattern::LastLongUnitOfMonthYear, |c, v, r| {
        c.offset_range_year_month(v.get_unit(0), v.get_int(2), v.get_int(1), convert::Change::Last)?
            .rule_time_reset(r)
    }),
    (&Pattern::LastLongUnitOfYear, |c, v, r| {
        c.offset_range_year_month(v.get_unit(0), v.get_int(1), 12, convert::Change::Last)?
            .rule_time_reset(r)
    }),
    (&Pattern::FirstLongUnitOfThisLongUnit, |c, v, r| {
        c.offset_range_unit(v.get_unit(0), v.get_unit(1), convert::Change::First)?
            .rule_time_reset(r)
    }),
    (&Pattern::LastLongUnitOfThisLongUnit, |c, v, r| {
        c.offset_range_unit(v.get_unit(0), v.get_unit(1), convert::Change::Last)?
            .rule_time_reset(r)
    }),
    (&Pattern::FirstLongUnitOfPrevLongUnit, |c, v, r| {
        c.offset_unit_keyword(v.get_unit(1), -1, r)?
            .offset_range_unit(v.get_unit(0), v.get_unit(1), convert::Change::First)?
            .rule_time_reset(r)
    }),
    (&Pattern::LastLongUnitOfPrevLongUnit, |c, v, r| {
        c.offset_unit_keyword(v.get_unit(1), -1, r)?
            .offset_range_unit(v.get_unit(0), v.get_unit(1), convert::Change::Last)?
            .rule_time_reset(r)
    }),
    (&Pattern::FirstLongUnitOfNextLongUnit, |c, v, r| {
        c.offset_unit_keyword(v.get_unit(1), 1, r)?
            .offset_range_unit(v.get_unit(0), v.get_unit(1), convert::Change::First)?
            .rule_time_reset(r)
    }),
    (&Pattern::LastLongUnitOfNextLongUnit, |c, v, r| {
        c.offset_unit_keyword(v.get_unit(1), 1, r)?
            .offset_range_unit(v.get_unit(0), v.get_unit(1), convert::Change::Last)?
            .rule_time_reset(r)
    }),
    // FIRST/LAST WEEKDAY OFFSETS
    (&Pattern::FirstWdayOfMonthYear, |c, v, r| {
        c.offset_range_year_month_wday(v.get_int(2), v.get_int(1), v.get_int(0), convert::Change::First)?
            .rule_time_reset(r)
    }),
    (&Pattern::FirstWdayOfMonth, |c, v, r| {
        c.offset_range_year_month_wday(c.rule_year(), v.get_int(1), v.get_int(0), convert::Change::First)?
            .rule_time_reset(r)
    }),
    (&Pattern::FirstWdayOfYear, |c, v, r| {
        c.offset_range_year_month_wday(v.get_int(1), 1, v.get_int(0), convert::Change::First)?
            .rule_time_reset(r)
    }),
    (&Pattern::LastWdayOfMonthYear, |c, v, r| {
        c.offset_range_year_month_wday(v.get_int(2), v.get_int(1), v.get_int(0), convert::Change::Last)?
            .rule_time_reset(r)
    }),
    (&Pattern::LastWdayOfMonth, |c, v, r| {
        c.offset_range_year_month_wday(c.rule_year(), v.get_int(1), v.get_int(0), convert::Change::Last)?
            .rule_time_reset(r)
    }),
    (&Pattern::LastWdayOfYear, |c, v, r| {
        c.offset_range_year_month_wday(v.get_int(1), 12, v.get_int(0), convert::Change::Last)?
            .rule_time_reset(r)
    }),
    // 20230130
    (&Pattern::Integer, |c, v, r| c.rule_allow_year_dates(r)?.date_iso8601(v.get_string(0))?.rule_time_reset(r)),
    // 2023
    (&Pattern::Year, |c, v, _| c.date_ym(v.get_int(0), c.month())),
    // 2023-W13
    (&Pattern::YearWeek, |c, v, r| {
        c.rule_allow_year_dates(r)?
            .date_yw(v.get_int(0), v.get_int(1), r)?
            .rule_time_reset(r)
    }),
    // April, April 2023
    (&Pattern::Month, |c, v, r| c.date_ym(c.rule_year(), v.get_int(0))?.rule_time_reset(r)),
    (&Pattern::MonthYear, |c, v, r| {
        c.rule_allow_year_dates(r)?
            .date_ymd(v.get_int(1), v.get_int(0), 1)?
            .rule_time_reset(r)
    }),
    // @1705072948, @1705072948.452
    (&Pattern::Timestamp, |c, v, r| c.rule_allow_year_dates(r)?.date_stamp(v.get_int(0), 0)),
    (&Pattern::TimestampFloat, |c, v, r| c.rule_allow_year_dates(r)?.date_stamp(v.get_int(0), v.get_ms(1))),
    // 2023-01-30, 30.1.2023, 1/30/2023
    (&Pattern::DateYmd, |c, v, r| {
        c.rule_allow_year_dates(r)?
            .date_ymd(v.get_int(0), v.get_int(1), v.get_int(2))?
            .rule_time_reset(r)
    }),
    (&Pattern::DateDmy, |c, v, r| {
        c.rule_allow_year_dates(r)?
            .date_ymd(v.get_int(2), v.get_int(1), v.get_int(0))?
            .rule_time_reset(r)
    }),
    (&Pattern::DateMdy, |c, v, r| {
        c.rule_allow_year_dates(r)?
            .date_ymd(v.get_int(2), v.get_int(0), v.get_int(1))?
            .rule_time_reset(r)
    }),
    // Dec 7, Dec 7th, 7 Dec
    (&Pattern::DateMonthDay, |c, v, r| c.date_ymd(c.rule_year(), v.get_int(0), v.get_int(1))?.rule_time_reset(r)),
    (&Pattern::DateMonthNth, |c, v, r| c.date_ymd(c.rule_year(), v.get_int(0), v.get_int(1))?.rule_time_reset(r)),
    (&Pattern::DateDayMonth, |c, v, r| c.date_ymd(c.rule_year(), v.get_int(1), v.get_int(0))?.rule_time_reset(r)),
    // Dec 7 2023, Dec 7th 2023, 7 Dec 2023
    (&Pattern::DateMonthDayYear, |c, v, r| {
        c.rule_allow_year_dates(r)?
            .date_ymd(v.get_int(2), v.get_int(0), v.get_int(1))?
            .rule_time_reset(r)
    }),
    (&Pattern::DateMonthNthYear, |c, v, r| {
        c.rule_allow_year_dates(r)?
            .date_ymd(v.get_int(2), v.get_int(0), v.get_int(1))?
            .rule_time_reset(r)
    }),
    (&Pattern::DateDayMonthYear, |c, v, r| {
        c.rule_allow_year_dates(r)?
            .date_ymd(v.get_int(2), v.get_int(1), v.get_int(0))?
            .rule_time_reset(r)
    }),
    // Thu, 7 Dec
    (&Pattern::DateWdayDayMonth, |c, v, r| {
        c.date_ymd(c.rule_year(), v.get_int(2), v.get_int(1))?
            .ensure_wday(v.get_int(0))?
            .rule_time_reset(r)
    }),
    // Thu, 7 Dec 2023
    (&Pattern::DateWdayDayMonthYear, |c, v, r| {
        c.rule_allow_year_dates(r)?
            .date_ymd(v.get_int(3), v.get_int(2), v.get_int(1))?
            .ensure_wday(v.get_int(0))?
            .rule_time_reset(r)
    }),
    // Thu, Dec 7th
    (&Pattern::DateWdayMontDay, |c, v, r| {
        c.date_ymd(c.rule_year(), v.get_int(1), v.get_int(2))?
            .ensure_wday(v.get_int(0))?
            .rule_time_reset(r)
    }),
    // Thu, Dec 7th 2023
    (&Pattern::DateWdayMontDayYear, |c, v, r| {
        c.rule_allow_year_dates(r)?
            .date_ymd(v.get_int(3), v.get_int(1), v.get_int(2))?
            .ensure_wday(v.get_int(0))?
            .rule_time_reset(r)
    }),
    // 2023-12-07 15:02:01, 2023-12-07 15:02:01.456
    (&Pattern::DateTimeYmdHms, |c, v, r| {
        c.rule_allow_year_dates(r)?
            .date_ymd(v.get_int(0), v.get_int(1), v.get_int(2))?
            .time_hms(v.get_int(3), v.get_int(4), v.get_int(5), 0)
    }),
    (&Pattern::DateTimeYmdHmsMs, |c, v, r| {
        c.rule_allow_year_dates(r)?
            .date_ymd(v.get_int(0), v.get_int(1), v.get_int(2))?
            .time_hms(v.get_int(3), v.get_int(4), v.get_int(5), v.get_ms(6))
    }),
    // 3:00, 3:00:00, 3:00:00.456
    (&Pattern::TimeHm, |c, v, _| c.time_hms(v.get_int(0), v.get_int(1), 0, 0)),
    (&Pattern::TimeHms, |c, v, _| c.time_hms(v.get_int(0), v.get_int(1), v.get_int(2), 0)),
    (&Pattern::TimeHmsMs, |c, v, _| c.time_hms(v.get_int(0), v.get_int(1), v.get_int(2), v.get_ms(3))),
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

struct CallSequence {
    calls: Vec<CallPattern>,
    mapping: HashMap<Pattern, Vec<usize>>,
    patterns: HashSet<Pattern>,
}

impl CallSequence {
    fn new(calls: Vec<CallPattern>) -> Self {
        let mut mapping: HashMap<Pattern, Vec<usize>> = HashMap::new();

        calls
            .iter()
            .enumerate()
            .for_each(|(i, v)| mapping.entry(v.pattern_type.to_owned()).or_insert(Vec::new()).push(i));

        let patterns = mapping.keys().cloned().collect();
        Self { calls: calls, mapping: mapping, patterns: patterns }
    }

    fn get_allowed(&self) -> Vec<Pattern> {
        if self.patterns.contains(&Pattern::Wday) {
            return Self::allowed_wday();
        }

        Vec::new()
    }

    fn get_default_year(&self, values: &CallValues) -> Option<i64> {
        // Whenever pattern for explicit and separate year is used, we
        // should not allow using patterns that contain another year, e.g.
        // preventing strings like "2025 Feb 20th 2026" from being used.
        let Some(index) = self.mapping.get(&Pattern::Year) else {
            return None;
        };

        let Some(call) = self.calls.get(*index.first().unwrap()) else {
            return None;
        };

        Some(values.get_int(call.value_offset))
    }

    fn has_pattern(&self, any_of: Vec<Pattern>) -> bool {
        let allowed = HashSet::from_iter(any_of);
        self.patterns.intersection(&allowed).count().gt(&0)
    }

    fn should_reset_time(&self) -> bool {
        // Whenever pattern for explicit time of day is given, we should
        // not reset time of day at the end of date movement, e.g. "today"
        // will reset time to midnight, while "2pm today" will not.
        !self.has_pattern(Vec::from(Pattern::time_of_days()))
    }

    fn sort(&mut self) {
        if self.calls.len().le(&1) {
            return;
        }

        let order = self.get_allowed();

        if order.is_empty() {
            return;
        }

        let order = order
            .iter()
            .enumerate()
            .map(|(i, v)| (v.to_owned(), i + 1))
            .collect::<HashMap<Pattern, usize>>();

        self.calls.sort_by(|a, b| {
            let a_index = order.get(&a.pattern_type).unwrap();
            let b_index = order.get(&b.pattern_type).unwrap();
            a_index.cmp(b_index)
        })
    }

    fn validate(&self) -> bool {
        if self.calls.len().le(&1) {
            return true;
        }

        // We can't have more than one separate year defined
        if let Some(indexes) = self.mapping.get(&Pattern::Year) {
            if indexes.len().gt(&1) {
                return false;
            }
        }

        let allowed = self.get_allowed();

        if allowed.is_empty() {
            return true;
        }

        let allowed = HashSet::from_iter(allowed);
        self.patterns.difference(&allowed).count().eq(&0)
    }

    fn allowed_wday() -> Vec<Pattern> {
        let mut result = Vec::from([
            Pattern::ThisLongUnit,
            Pattern::PastLongUnit,
            Pattern::PrevLongUnit,
            Pattern::NextLongUnit,
            Pattern::Wday,
        ]);
        result.extend(Pattern::time_of_days());
        result
    }
}

#[allow(dead_code)]
struct CallPattern {
    pattern_type: Pattern,
    pattern_match: String,
    callback: fn(FuzzyDate, &CallValues, &Rules) -> Result<FuzzyDate, ()>,
    value_offset: usize,
}

struct CallValues {
    position: usize,
    tokens: Vec<Token>,
}

impl CallValues {
    fn from_tokens(tokens: Vec<Token>) -> Self {
        Self { position: 0, tokens: tokens }
    }

    fn get_int(&self, index: usize) -> i64 {
        let index = self.position + index;
        self.tokens[index].value
    }

    fn get_string(&self, index: usize) -> String {
        let index = self.position + index;
        let value = self.tokens[index].value;
        let zeros = self.tokens[index].zeros;
        format!("{}{}", "0".repeat(zeros as usize), value)
    }

    /// Get value with the assumption that it should represent milliseconds,
    /// and thus the number of zeros before the number is meaningful. If there
    /// are too many zeros, we use -1 to break out on millisecond value
    /// validation.
    fn get_ms(&self, index: usize) -> i64 {
        let index = self.position + index;
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
    default_year: Option<i64>,
    time: DateTime<FixedOffset>,
}

impl FuzzyDate {
    /// Get a new instance of self with defaults
    fn with_defaults(&self, new_time: DateTime<FixedOffset>) -> Self {
        Self { default_year: self.default_year, time: new_time }
    }

    /// Get a new instance of self without defaults
    fn without_defaults(&self, new_time: DateTime<FixedOffset>) -> Self {
        Self { default_year: None, time: new_time }
    }

    /// Set time to specific data from basic ISO8601 date string
    fn date_iso8601(&self, value: String) -> Result<Self, ()> {
        Ok(self.with_defaults(convert::date_iso8601(self.time, value)?))
    }

    /// Set time to specific timestamp
    fn date_stamp(&self, sec: i64, ms: i64) -> Result<Self, ()> {
        Ok(self.with_defaults(convert::date_stamp(sec, ms)))
    }

    /// Set time to specific year and week number
    fn date_yw(&self, year: i64, week: i64, rules: &Rules) -> Result<Self, ()> {
        Ok(self.without_defaults(convert::date_yw(self.time, year, week, rules.week_start_day())?))
    }

    /// Set time to specific year and month
    fn date_ym(&self, year: i64, month: i64) -> Result<Self, ()> {
        let month_day = convert::into_month_day(year as i32, month as u32, self.time.day());
        Ok(self.without_defaults(convert::date_ymd(self.time, year, month, month_day as i64)?))
    }

    /// Set time to specific year, month and day
    fn date_ymd(&self, year: i64, month: i64, day: i64) -> Result<Self, ()> {
        Ok(self.without_defaults(convert::date_ymd(self.time, year, month, day)?))
    }

    /// Ensure that given value matches to allowed unit
    fn ensure_unit(&self, given: TimeUnit, accept: TimeUnit) -> Result<Self, ()> {
        match given.eq(&accept) {
            true => Ok(self.with_defaults(self.time)),
            false => Err(()),
        }
    }

    /// Ensure that the date has specified weekday
    pub(crate) fn ensure_wday(&self, wday: i64) -> Result<Self, ()> {
        match self.time.weekday().number_from_monday().eq(&(wday as u32)) {
            true => Ok(self.with_defaults(self.time)),
            false => Err(()),
        }
    }

    /// Current month
    fn month(&self) -> i64 {
        self.time.month() as i64
    }

    /// Move time into current or upcoming weekday
    fn offset_current_weekday(&self, new_weekday: i64) -> Result<Self, ()> {
        match self.weekday().eq(&new_weekday) {
            true => Ok(self.with_defaults(self.time)),
            false => self.offset_weekday(new_weekday, convert::Change::Next),
        }
    }

    /// Move time into previous or upcoming month
    fn offset_month(&self, new_month: i64, change: convert::Change) -> Result<Self, ()> {
        Ok(self.with_defaults(convert::offset_month(self.time, new_month, change)))
    }

    /// Move time into previous or upcoming weekday
    fn offset_weekday(&self, new_weekday: i64, change: convert::Change) -> Result<Self, ()> {
        Ok(self.with_defaults(convert::offset_weekday(self.time, new_weekday, change)))
    }

    /// Move time within month range
    fn offset_range_month(&self, target: TimeUnit, month: i64, change: convert::Change) -> Result<Self, ()> {
        if target.eq(&TimeUnit::Days) {
            let new_time = convert::offset_range_year_month(self.time, self.time.year() as i64, month, change)?;
            return Ok(self.with_defaults(new_time));
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
                return Ok(self.with_defaults(self.time.with_day(last_day).unwrap()));
            }

            return Ok(self.with_defaults(self.time.with_day(1).unwrap()));
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

        Ok(self.with_defaults(new_time))
    }

    /// Move time by specific unit, but apply keyword rules where
    /// e.g. moving by weeks will land on to first day of week
    fn offset_unit_keyword(&self, target: TimeUnit, amount: i64, rules: &Rules) -> Result<FuzzyDate, ()> {
        let new_time = match target {
            TimeUnit::Weeks => convert::offset_weeks(self.time, amount, rules.week_start_day()),
            _ => return self.offset_unit_exact(target, amount, rules),
        };

        Ok(self.with_defaults(new_time))
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
            return Ok(self.with_defaults(new_time));
        }

        Err(())
    }

    /// Move time to a weekday within year and month range
    pub(crate) fn offset_range_year_month_wday(
        &self,
        year: i64,
        month: i64,
        wday: i64,
        change: convert::Change,
    ) -> Result<Self, ()> {
        let new_time = convert::offset_range_year_month_wday(self.time, year, month, wday, change)?;
        Ok(self.without_defaults(new_time))
    }

    /// Ensure that rules allow changing the year
    fn rule_allow_year_dates(&self, rules: &Rules) -> Result<Self, ()> {
        match rules.date_years {
            true => Ok(self.with_defaults(self.time)),
            false => Err(()),
        }
    }

    /// Get default year (once) from rules, or current year
    fn rule_year(&self) -> i64 {
        self.default_year.unwrap_or(self.time.year() as i64)
    }

    /// Reset time to midnight, if rules allow it
    fn rule_time_reset(&self, rules: &Rules) -> Result<Self, ()> {
        match rules.reset_time {
            true => self.time_hms(0, 0, 0, 0),
            false => Ok(self.with_defaults(self.time)),
        }
    }

    /// Set time to specific hour, minute and second using 12-hour clock
    fn time_12h(&self, hour: i64, min: i64, sec: i64, meridiem: i64) -> Result<Self, ()> {
        Ok(self.with_defaults(convert::time_12h(self.time, hour, min, sec, meridiem)?))
    }

    /// Set time to specific hour, minute and second
    fn time_hms(&self, hour: i64, min: i64, sec: i64, ms: i64) -> Result<Self, ()> {
        Ok(self.with_defaults(convert::time_hms(self.time, hour, min, sec, ms)?))
    }

    /// Current weekday, matching to token values
    fn weekday(&self) -> i64 {
        self.time.weekday().num_days_from_monday() as i64 + 1
    }
}

struct Rules {
    date_years: bool,
    reset_time: bool,
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
    let mut call_sequence = CallSequence::new(call_list);

    if call_sequence.calls.is_empty() {
        return None;
    }

    if !call_sequence.validate() {
        return None;
    }

    call_sequence.sort();

    let mut ctx_vals = CallValues::from_tokens(tokens);
    let mut ctx_time =
        FuzzyDate { time: current_time.to_owned(), default_year: call_sequence.get_default_year(&ctx_vals) };

    let rules = Rules {
        date_years: ctx_time.default_year.is_none(),
        reset_time: call_sequence.should_reset_time(),
        week_start_mon: week_start_mon,
    };

    for item in call_sequence.calls {
        ctx_vals.position = item.value_offset;
        ctx_time = match (item.callback)(ctx_time, &ctx_vals, &rules) {
            Ok(value) => value,
            Err(_) => return None,
        };
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
fn find_pattern_calls(pattern: &str, custom: HashMap<String, String>) -> Vec<CallPattern> {
    let closure_map: HashMap<&Pattern, fn(FuzzyDate, &CallValues, &Rules) -> Result<FuzzyDate, ()>> =
        HashMap::from(FUZZY_PATTERNS);

    let pattern_keys = closure_map.keys().map(|v| v.to_owned()).collect::<HashSet<&Pattern>>();
    let mut pattern_map = Pattern::value_patterns(pattern_keys);

    for (custom_pattern, closure_pattern) in custom.iter() {
        if let Some(pattern_constant) = pattern_map.get(closure_pattern) {
            pattern_map.insert(custom_pattern.to_owned(), pattern_constant.to_owned());
        }
    }

    for prefix in vec!["", "+"] {
        let try_pattern = format!("{}{}", prefix, pattern);

        if let Some(pattern_type) = pattern_map.get(&try_pattern) {
            return Vec::from([CallPattern {
                pattern_type: pattern_type.to_owned(),
                pattern_match: try_pattern.to_owned(),
                callback: *closure_map.get(pattern_type).unwrap(),
                value_offset: 0,
            }]);
        }
    }

    let prefix = find_pattern_prefix(pattern, custom);

    let mut result = Vec::new();
    let mut value_offset = 0;
    let mut search = pattern;

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

        result.push(CallPattern {
            pattern_type: (*best_type).to_owned(),
            pattern_match: best_match.to_string(),
            callback: *closure_map.get(best_type).unwrap(),
            value_offset: value_offset,
        });

        value_offset += best_match.split("[").count() - 1;
    }

    result
}

/// Figure out whether unit lengths in pattern are negative or positive
fn find_pattern_prefix(pattern: &str, custom: HashMap<String, String>) -> &'static str {
    if pattern.starts_with("-") {
        return "-";
    }

    if pattern.starts_with("+") || !pattern.contains("unit]") {
        return "+";
    }

    // Check whether the pattern ending matches with an "ago" pattern in a
    // from both internal and custom patterns, to prefer using minus patterns
    for pattern_type in vec![Pattern::LongUnitAgo, Pattern::UnitAgo] {
        for pattern_value in Pattern::values(&pattern_type) {
            if pattern.ends_with(pattern_value) {
                return "-";
            }

            for (custom_pattern, closure_pattern) in custom.iter() {
                if closure_pattern.eq(pattern_value) && pattern.ends_with(custom_pattern) {
                    return "-";
                }
            }
        }
    }

    "+"
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
            ("[int] [long_unit] sitten", &Pattern::LongUnitAgo),
        ];

        let result_value = convert_custom("viime [wday]", vec![1], "2024-01-19T15:22:28+02:00", &custom_finnish);
        assert_eq!(result_value, "2024-01-15 00:00:00 +02:00");

        let result_value = convert_custom("edellinen [wday]", vec![1], "2024-01-19T15:22:28+02:00", &custom_finnish);
        assert_eq!(result_value, "2024-01-15 00:00:00 +02:00");

        let result_value = convert_custom("ensi [wday]", vec![1], "2024-01-19T15:22:28+02:00", &custom_finnish);
        assert_eq!(result_value, "2024-01-22 00:00:00 +02:00");

        let result_value = convert_custom("seuraava [wday]", vec![1], "2024-01-19T15:22:28+02:00", &custom_finnish);
        assert_eq!(result_value, "2024-01-22 00:00:00 +02:00");

        let token_values = vec![1, 4, 1, 3]; // 1d 1h
        let result_value = convert_custom(
            "[int] [long_unit] [int] [long_unit] sitten",
            token_values,
            "2024-01-19T15:22:28+02:00",
            &custom_finnish,
        );
        assert_eq!(result_value, "2024-01-18 14:22:28 +02:00");
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
