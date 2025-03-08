// PATTERNS

use std::collections::{HashMap, HashSet};

pub(crate) const PATTERN_NOW: &'static str = "now";
pub(crate) const PATTERN_TODAY: &'static str = "today";
pub(crate) const PATTERN_MIDNIGHT: &'static str = "midnight";
pub(crate) const PATTERN_YESTERDAY: &'static str = "yesterday";
pub(crate) const PATTERN_TOMORROW: &'static str = "tomorrow";

pub(crate) const PATTERN_WDAY: &'static str = "[wday]";
pub(crate) const PATTERN_THIS_WDAY: &'static str = "this [wday]";
pub(crate) const PATTERN_PREV_WDAY: &'static str = "prev [wday]";
pub(crate) const PATTERN_LAST_WDAY: &'static str = "last [wday]";
pub(crate) const PATTERN_NEXT_WDAY: &'static str = "next [wday]";

pub(crate) const PATTERN_THIS_MONTH: &'static str = "this [month]";
pub(crate) const PATTERN_PREV_MONTH: &'static str = "prev [month]";
pub(crate) const PATTERN_LAST_MONTH: &'static str = "last [month]";
pub(crate) const PATTERN_NEXT_MONTH: &'static str = "next [month]";

pub(crate) const PATTERN_THIS_LONG_UNIT: &'static str = "this [long_unit]";
pub(crate) const PATTERN_PAST_LONG_UNIT: &'static str = "past [long_unit]";
pub(crate) const PATTERN_PREV_LONG_UNIT: &'static str = "prev [long_unit]";
pub(crate) const PATTERN_LAST_LONG_UNIT: &'static str = "last [long_unit]";
pub(crate) const PATTERN_NEXT_LONG_UNIT: &'static str = "next [long_unit]";

pub(crate) const PATTERN_MINUS_UNIT: &'static str = "-[int][unit]";
pub(crate) const PATTERN_MINUS_SHORT_UNIT: &'static str = "-[int][short_unit]";
pub(crate) const PATTERN_MINUS_LONG_UNIT: &'static str = "-[int] [long_unit]";

pub(crate) const PATTERN_PAST_N_LONG_UNIT: &'static str = "past [int] [long_unit]";
pub(crate) const PATTERN_PREV_N_LONG_UNIT: &'static str = "prev [int] [long_unit]";
pub(crate) const PATTERN_LAST_N_LONG_UNIT: &'static str = "last [int] [long_unit]";

pub(crate) const PATTERN_PLUS_UNIT: &'static str = "+[int][unit]";
pub(crate) const PATTERN_PLUS_SHORT_UNIT: &'static str = "+[int][short_unit]";
pub(crate) const PATTERN_PLUS_LONG_UNIT: &'static str = "+[int] [long_unit]";
pub(crate) const PATTERN_UNIT_AGO: &'static str = "[int] [unit] ago";
pub(crate) const PATTERN_LONG_UNIT_AGO: &'static str = "[int] [long_unit] ago";
pub(crate) const PATTERN_LONG_UNIT_INT: &'static str = "[long_unit] [int]";
pub(crate) const PATTERN_LONG_UNIT_INT_YEAR: &'static str = "[long_unit] [int] [year]";

pub(crate) const PATTERN_FIRST_LONG_UNIT_OF_MONTH: &'static str = "first [long_unit] of [month]";
pub(crate) const PATTERN_FIRST_LONG_UNIT_OF_MONTH_YEAR: &'static str = "first [long_unit] of [month] [year]";
pub(crate) const PATTERN_FIRST_LONG_UNIT_OF_YEAR: &'static str = "first [long_unit] of [year]";
pub(crate) const PATTERN_LAST_LONG_UNIT_OF_MONTH: &'static str = "last [long_unit] of [month]";
pub(crate) const PATTERN_LAST_LONG_UNIT_OF_MONTH_YEAR: &'static str = "last [long_unit] of [month] [year]";
pub(crate) const PATTERN_LAST_LONG_UNIT_OF_YEAR: &'static str = "last [long_unit] of [year]";

pub(crate) const PATTERN_FIRST_WDAY_OF_MONTH: &'static str = "first [wday] of [month]";
pub(crate) const PATTERN_FIRST_WDAY_OF_MONTH_YEAR: &'static str = "first [wday] of [month] [year]";
pub(crate) const PATTERN_FIRST_WDAY_OF_YEAR: &'static str = "first [wday] of [year]";
pub(crate) const PATTERN_LAST_WDAY_OF_MONTH: &'static str = "last [wday] of [month]";
pub(crate) const PATTERN_LAST_WDAY_OF_MONTH_YEAR: &'static str = "last [wday] of [month] [year]";
pub(crate) const PATTERN_LAST_WDAY_OF_YEAR: &'static str = "last [wday] of [year]";

pub(crate) const PATTERN_FIRST_OF_LONG_UNIT: &'static str = "first of [long_unit]";
pub(crate) const PATTERN_FIRST_OF_THE_LONG_UNIT: &'static str = "first of the [long_unit]";
pub(crate) const PATTERN_FIRST_OF_THIS_LONG_UNIT: &'static str = "first of this [long_unit]";
pub(crate) const PATTERN_LAST_OF_LONG_UNIT: &'static str = "last of [long_unit]";
pub(crate) const PATTERN_LAST_OF_THE_LONG_UNIT: &'static str = "last of the [long_unit]";
pub(crate) const PATTERN_LAST_OF_THIS_LONG_UNIT: &'static str = "last of this [long_unit]";

pub(crate) const PATTERN_FIRST_LONG_UNIT_OF_THIS_LONG_UNIT: &'static str = "first [long_unit] of this [long_unit]";
pub(crate) const PATTERN_LAST_LONG_UNIT_OF_THIS_LONG_UNIT: &'static str = "last [long_unit] of this [long_unit]";
pub(crate) const PATTERN_FIRST_LONG_UNIT_OF_PREV_LONG_UNIT: &'static str = "first [long_unit] of prev [long_unit]";
pub(crate) const PATTERN_LAST_LONG_UNIT_OF_PREV_LONG_UNIT: &'static str = "last [long_unit] of prev [long_unit]";
pub(crate) const PATTERN_FIRST_LONG_UNIT_OF_LAST_LONG_UNIT: &'static str = "first [long_unit] of last [long_unit]";
pub(crate) const PATTERN_LAST_LONG_UNIT_OF_LAST_LONG_UNIT: &'static str = "last [long_unit] of last [long_unit]";
pub(crate) const PATTERN_FIRST_LONG_UNIT_OF_NEXT_LONG_UNIT: &'static str = "first [long_unit] of next [long_unit]";
pub(crate) const PATTERN_LAST_LONG_UNIT_OF_NEXT_LONG_UNIT: &'static str = "last [long_unit] of next [long_unit]";

pub(crate) const PATTERN_INTEGER: &'static str = "[int]";
pub(crate) const PATTERN_MONTH: &'static str = "[month]";
pub(crate) const PATTERN_MONTH_YEAR: &'static str = "[month] [year]";

pub(crate) const PATTERN_TIMESTAMP: &'static str = "[timestamp]";
pub(crate) const PATTERN_TIMESTAMP_FLOAT: &'static str = "[timestamp].[int]";

pub(crate) const PATTERN_YEAR: &'static str = "[year]";

pub(crate) const PATTERN_YEAR_WEEK: &'static str = "[year]-W[int]";
pub(crate) const PATTERN_YW: &'static str = "[year]W[int]";

pub(crate) const PATTERN_DATE_YMD: &'static str = "[year]-[int]-[int]";
pub(crate) const PATTERN_DATE_DMY: &'static str = "[int].[int].[year]";
pub(crate) const PATTERN_DATE_MDY: &'static str = "[int]/[int]/[year]";

pub(crate) const PATTERN_DATE_MONTH_DAY: &'static str = "[month] [int]";
pub(crate) const PATTERN_DATE_MONTH_DAY_YEAR: &'static str = "[month] [int] [year]";
pub(crate) const PATTERN_DATE_MONTH_DAY_YEAR_DASHED: &'static str = "[month]-[int]-[year]";
pub(crate) const PATTERN_DATE_MONTH_NTH: &'static str = "[month] [nth]";
pub(crate) const PATTERN_DATE_MONTH_NTH_YEAR: &'static str = "[month] [nth] [year]";
pub(crate) const PATTERN_DATE_DAY_MONTH: &'static str = "[int] [month]";
pub(crate) const PATTERN_DATE_DAY_MONTH_YEAR: &'static str = "[int] [month] [year]";
pub(crate) const PATTERN_DATE_DAY_MONTH_YEAR_DASHED: &'static str = "[int]-[month]-[year]";
pub(crate) const PATTERN_DATE_NTH_MONTH: &'static str = "[nth] [month]";
pub(crate) const PATTERN_DATE_NTH_MONTH_YEAR: &'static str = "[nth] [month] [year]";
pub(crate) const PATTERN_DATE_NTH_OF_MONTH: &'static str = "[nth] of [month]";
pub(crate) const PATTERN_DATE_NTH_OF_MONTH_YEAR: &'static str = "[nth] of [month] [year]";
pub(crate) const PATTERN_DATE_YEAR_MONTH_DAY_DASHED: &'static str = "[year]-[month]-[int]";

pub(crate) const PATTERN_DATE_WDAY_DAY_MONTH: &'static str = "[wday] [int] [month]";
pub(crate) const PATTERN_DATE_WDAY_DAY_MONTH_YEAR: &'static str = "[wday] [int] [month] [year]";
pub(crate) const PATTERN_DATE_WDAY_MONTH_DAY: &'static str = "[wday] [month] [int]";
pub(crate) const PATTERN_DATE_WDAY_MONTH_DAY_HMS_YEAR: &'static str = "[wday] [month] [int] [int]:[int]:[int] [year]";
pub(crate) const PATTERN_DATE_WDAY_MONTH_NTH: &'static str = "[wday] [month] [nth]";
pub(crate) const PATTERN_DATE_WDAY_MONTH_NTH_YEAR: &'static str = "[wday] [month] [nth] [year]";
pub(crate) const PATTERN_DATE_WDAY_MONTH_YEAR: &'static str = "[wday] [month] [int] [year]";
pub(crate) const PATTERN_DATE_WDAY_NTH_MONTH: &'static str = "[wday] [nth] [month]";
pub(crate) const PATTERN_DATE_WDAY_NTH_MONTH_YEAR: &'static str = "[wday] [nth] [month] [year]";
pub(crate) const PATTERN_DATE_WDAY_NTH_OF_MONTH: &'static str = "[wday] [nth] of [month]";
pub(crate) const PATTERN_DATE_WDAY_NTH_OF_MONTH_YEAR: &'static str = "[wday] [nth] of [month] [year]";

pub(crate) const PATTERN_DATETIME_ISO_YMD_HMS: &'static str = "[year]-[int]-[int]T[int]:[int]:[int]";
pub(crate) const PATTERN_DATETIME_ISO_YMD_HMS_MS: &'static str = "[year]-[int]-[int]T[int]:[int]:[int].[int]";

pub(crate) const PATTERN_DATETIME_YMD_HMS: &'static str = "[year]-[int]-[int] [int]:[int]:[int]";
pub(crate) const PATTERN_DATETIME_YMD_HMS_MS: &'static str = "[year]-[int]-[int] [int]:[int]:[int].[int]";

pub(crate) const PATTERN_TIME_HM: &'static str = "[int]:[int]";
pub(crate) const PATTERN_TIME_HMS: &'static str = "[int]:[int]:[int]";
pub(crate) const PATTERN_TIME_HMS_MS: &'static str = "[int]:[int]:[int].[int]";
pub(crate) const PATTERN_TIME_12H_H: &'static str = "[int] [meridiem]";
pub(crate) const PATTERN_TIME_12H_HM: &'static str = "[int]:[int] [meridiem]";
pub(crate) const PATTERN_TIME_12H_HOUR: &'static str = "[int][meridiem]";

pub(crate) const PATTERN_TIME_AT_HM: &'static str = "at [int]:[int]";
pub(crate) const PATTERN_TIME_AT_HMS: &'static str = "at [int]:[int]:[int]";
pub(crate) const PATTERN_TIME_AT_HMS_MS: &'static str = "at [int]:[int]:[int].[int]";
pub(crate) const PATTERN_TIME_AT_12H_H: &'static str = "at [int] [meridiem]";
pub(crate) const PATTERN_TIME_AT_12H_HM: &'static str = "at [int]:[int] [meridiem]";
pub(crate) const PATTERN_TIME_AT_12H_HOUR: &'static str = "at [int][meridiem]";

pub(crate) const PATTERN_TIME_AT_SIGN_HM: &'static str = "@ [int]:[int]";
pub(crate) const PATTERN_TIME_AT_SIGN_HMS: &'static str = "@ [int]:[int]:[int]";
pub(crate) const PATTERN_TIME_AT_SIGN_HMS_MS: &'static str = "@ [int]:[int]:[int].[int]";
pub(crate) const PATTERN_TIME_AT_SIGN_12H_H: &'static str = "@ [int] [meridiem]";
pub(crate) const PATTERN_TIME_AT_SIGN_12H_HM: &'static str = "@ [int]:[int] [meridiem]";
pub(crate) const PATTERN_TIME_AT_SIGN_12H_HOUR: &'static str = "@ [int][meridiem]";

// TOKENS

// Weekdays
pub(crate) const TOKEN_WDAY_MON: i16 = 101;
pub(crate) const TOKEN_WDAY_TUE: i16 = 102;
pub(crate) const TOKEN_WDAY_WED: i16 = 103;
pub(crate) const TOKEN_WDAY_THU: i16 = 104;
pub(crate) const TOKEN_WDAY_FRI: i16 = 105;
pub(crate) const TOKEN_WDAY_SAT: i16 = 106;
pub(crate) const TOKEN_WDAY_SUN: i16 = 107;

// Months
pub(crate) const TOKEN_MONTH_JAN: i16 = 201;
pub(crate) const TOKEN_MONTH_FEB: i16 = 202;
pub(crate) const TOKEN_MONTH_MAR: i16 = 203;
pub(crate) const TOKEN_MONTH_APR: i16 = 204;
pub(crate) const TOKEN_MONTH_MAY: i16 = 205;
pub(crate) const TOKEN_MONTH_JUN: i16 = 206;
pub(crate) const TOKEN_MONTH_JUL: i16 = 207;
pub(crate) const TOKEN_MONTH_AUG: i16 = 208;
pub(crate) const TOKEN_MONTH_SEP: i16 = 209;
pub(crate) const TOKEN_MONTH_OCT: i16 = 210;
pub(crate) const TOKEN_MONTH_NOV: i16 = 211;
pub(crate) const TOKEN_MONTH_DEC: i16 = 212;

pub(crate) const TOKEN_UNIT_SEC: i16 = 301;
pub(crate) const TOKEN_UNIT_MIN: i16 = 302;
pub(crate) const TOKEN_UNIT_HRS: i16 = 303;

pub(crate) const TOKEN_SHORT_UNIT_SEC: i16 = 401;
pub(crate) const TOKEN_SHORT_UNIT_HRS: i16 = 403;
pub(crate) const TOKEN_SHORT_UNIT_DAY: i16 = 404;
pub(crate) const TOKEN_SHORT_UNIT_WEEK: i16 = 405;
pub(crate) const TOKEN_SHORT_UNIT_MONTH: i16 = 406;
pub(crate) const TOKEN_SHORT_UNIT_YEAR: i16 = 407;

pub(crate) const TOKEN_LONG_UNIT_SEC: i16 = 501;
pub(crate) const TOKEN_LONG_UNIT_MIN: i16 = 502;
pub(crate) const TOKEN_LONG_UNIT_HRS: i16 = 503;
pub(crate) const TOKEN_LONG_UNIT_DAY: i16 = 504;
pub(crate) const TOKEN_LONG_UNIT_WEEK: i16 = 505;
pub(crate) const TOKEN_LONG_UNIT_MONTH: i16 = 506;
pub(crate) const TOKEN_LONG_UNIT_YEAR: i16 = 507;

pub(crate) const TOKEN_MERIDIEM_AM: i16 = 601;
pub(crate) const TOKEN_MERIDIEM_PM: i16 = 602;

pub(crate) const UNIT_DAY: &'static str = "day";
pub(crate) const UNIT_DAYS: &'static str = "days";
pub(crate) const UNIT_HOUR: &'static str = "hour";
pub(crate) const UNIT_HOURS: &'static str = "hours";
pub(crate) const UNIT_MINUTE: &'static str = "minute";
pub(crate) const UNIT_MINUTES: &'static str = "minutes";
pub(crate) const UNIT_SECOND: &'static str = "second";
pub(crate) const UNIT_SECONDS: &'static str = "seconds";
pub(crate) const UNIT_WEEK: &'static str = "week";
pub(crate) const UNIT_WEEKS: &'static str = "weeks";

#[derive(Clone, PartialEq, Eq, Hash)]
pub(crate) enum Pattern {
    Integer,
    Month,
    MonthYear,

    Now,
    Today,
    Midnight,
    Yesterday,
    Tomorrow,

    Wday,
    ThisWday,
    PrevWday,
    NextWday,

    ThisMonth,
    PrevMonth,
    NextMonth,

    ThisLongUnit,
    PastLongUnit,
    PrevLongUnit,
    PrevNLongUnit,
    NextLongUnit,

    MinusUnit,
    MinusShortUnit,
    MinusLongUnit,

    PlusUnit,
    PlusShortUnit,
    PlusLongUnit,

    UnitAgo,

    LongUnitAgo,
    LongUnitInt,
    LongUnitIntYear,

    FirstOfLongUnit,
    FirstLongUnitOfMonth,
    FirstLongUnitOfMonthYear,
    FirstLongUnitOfYear,
    FirstLongUnitOfThisLongUnit,
    FirstLongUnitOfPrevLongUnit,
    FirstLongUnitOfNextLongUnit,

    FirstWdayOfMonth,
    FirstWdayOfMonthYear,
    FirstWdayOfYear,

    LastOfLongUnit,
    LastLongUnitOfMonth,
    LastLongUnitOfMonthYear,
    LastLongUnitOfYear,
    LastLongUnitOfThisLongUnit,
    LastLongUnitOfPrevLongUnit,
    LastLongUnitOfNextLongUnit,

    LastWdayOfMonth,
    LastWdayOfMonthYear,
    LastWdayOfYear,

    Timestamp,
    TimestampFloat,

    Year,
    YearWeek,

    DateYmd,
    DateDmy,
    DateMdy,
    DateMonthDayYear,
    DateMonthDay,
    DateMonthNth,
    DateMonthNthYear,
    DateDayMonth,
    DateDayMonthYear,
    DateTimeYmdHms,
    DateTimeYmdHmsMs,

    DateWdayDayMonth,
    DateWdayDayMonthYear,
    DateWdayMontDay,
    DateWdayMontDayYear,

    TimeHm,
    TimeHms,
    TimeHmsMs,
    TimeMeridiemH,
    TimeMeridiemHm,
}

impl Pattern {
    pub(crate) fn time_of_days() -> [Self; 5] {
        [
            Self::TimeHm,
            Self::TimeHms,
            Self::TimeHmsMs,
            Self::TimeMeridiemH,
            Self::TimeMeridiemHm,
        ]
    }

    /// Hashmap of string patterns mapped to constant values
    pub(crate) fn value_patterns(only_patterns: HashSet<&Pattern>) -> HashMap<String, Pattern> {
        let mut result = patterns()
            .iter()
            .map(|v| (v.1.to_string(), v.0.to_owned()))
            .collect::<HashMap<String, Pattern>>();

        result.retain(|_, v| only_patterns.contains(v));
        result
    }

    pub(crate) fn values(key: &Pattern) -> Vec<&'static str> {
        patterns().iter().filter(|&v| v.0.eq(&key)).map(|v| v.1).collect()
    }

    pub(crate) fn is_valid(value: &str) -> bool {
        patterns().iter().find(|&v| v.1 == value).is_some()
    }
}

fn patterns() -> [(Pattern, &'static str); 114] {
    [
        (Pattern::Integer, PATTERN_INTEGER),
        (Pattern::Month, PATTERN_MONTH),
        (Pattern::MonthYear, PATTERN_MONTH_YEAR),
        (Pattern::Now, PATTERN_NOW),
        (Pattern::Today, PATTERN_TODAY),
        (Pattern::Midnight, PATTERN_MIDNIGHT),
        (Pattern::Yesterday, PATTERN_YESTERDAY),
        (Pattern::Tomorrow, PATTERN_TOMORROW),
        (Pattern::Wday, PATTERN_WDAY),
        (Pattern::ThisWday, PATTERN_THIS_WDAY),
        (Pattern::PrevWday, PATTERN_PREV_WDAY),
        (Pattern::PrevWday, PATTERN_LAST_WDAY),
        (Pattern::NextWday, PATTERN_NEXT_WDAY),
        (Pattern::ThisMonth, PATTERN_THIS_MONTH),
        (Pattern::PrevMonth, PATTERN_PREV_MONTH),
        (Pattern::PrevMonth, PATTERN_LAST_MONTH),
        (Pattern::NextMonth, PATTERN_NEXT_MONTH),
        (Pattern::ThisLongUnit, PATTERN_THIS_LONG_UNIT),
        (Pattern::PastLongUnit, PATTERN_PAST_LONG_UNIT),
        (Pattern::PrevLongUnit, PATTERN_PREV_LONG_UNIT),
        (Pattern::PrevLongUnit, PATTERN_LAST_LONG_UNIT),
        (Pattern::NextLongUnit, PATTERN_NEXT_LONG_UNIT),
        (Pattern::MinusUnit, PATTERN_MINUS_UNIT),
        (Pattern::MinusShortUnit, PATTERN_MINUS_SHORT_UNIT),
        (Pattern::MinusLongUnit, PATTERN_MINUS_LONG_UNIT),
        (Pattern::MinusLongUnit, PATTERN_PAST_N_LONG_UNIT),
        (Pattern::PrevNLongUnit, PATTERN_PREV_N_LONG_UNIT),
        (Pattern::PrevNLongUnit, PATTERN_LAST_N_LONG_UNIT),
        (Pattern::PlusUnit, PATTERN_PLUS_UNIT),
        (Pattern::PlusShortUnit, PATTERN_PLUS_SHORT_UNIT),
        (Pattern::PlusLongUnit, PATTERN_PLUS_LONG_UNIT),
        (Pattern::UnitAgo, PATTERN_UNIT_AGO),
        (Pattern::LongUnitAgo, PATTERN_LONG_UNIT_AGO),
        (Pattern::LongUnitInt, PATTERN_LONG_UNIT_INT),
        (Pattern::LongUnitIntYear, PATTERN_LONG_UNIT_INT_YEAR),
        (Pattern::FirstOfLongUnit, PATTERN_FIRST_OF_LONG_UNIT),
        (Pattern::FirstOfLongUnit, PATTERN_FIRST_OF_THE_LONG_UNIT),
        (Pattern::FirstOfLongUnit, PATTERN_FIRST_OF_THIS_LONG_UNIT),
        (Pattern::FirstLongUnitOfMonth, PATTERN_FIRST_LONG_UNIT_OF_MONTH),
        (Pattern::FirstLongUnitOfMonthYear, PATTERN_FIRST_LONG_UNIT_OF_MONTH_YEAR),
        (Pattern::FirstLongUnitOfYear, PATTERN_FIRST_LONG_UNIT_OF_YEAR),
        (Pattern::FirstLongUnitOfThisLongUnit, PATTERN_FIRST_LONG_UNIT_OF_THIS_LONG_UNIT),
        (Pattern::FirstLongUnitOfPrevLongUnit, PATTERN_FIRST_LONG_UNIT_OF_PREV_LONG_UNIT),
        (Pattern::FirstLongUnitOfPrevLongUnit, PATTERN_FIRST_LONG_UNIT_OF_LAST_LONG_UNIT),
        (Pattern::FirstLongUnitOfNextLongUnit, PATTERN_FIRST_LONG_UNIT_OF_NEXT_LONG_UNIT),
        (Pattern::FirstWdayOfMonth, PATTERN_FIRST_WDAY_OF_MONTH),
        (Pattern::FirstWdayOfMonthYear, PATTERN_FIRST_WDAY_OF_MONTH_YEAR),
        (Pattern::FirstWdayOfYear, PATTERN_FIRST_WDAY_OF_YEAR),
        (Pattern::LastWdayOfMonth, PATTERN_LAST_WDAY_OF_MONTH),
        (Pattern::LastWdayOfMonthYear, PATTERN_LAST_WDAY_OF_MONTH_YEAR),
        (Pattern::LastWdayOfYear, PATTERN_LAST_WDAY_OF_YEAR),
        (Pattern::LastOfLongUnit, PATTERN_LAST_OF_LONG_UNIT),
        (Pattern::LastOfLongUnit, PATTERN_LAST_OF_THE_LONG_UNIT),
        (Pattern::LastOfLongUnit, PATTERN_LAST_OF_THIS_LONG_UNIT),
        (Pattern::LastLongUnitOfMonth, PATTERN_LAST_LONG_UNIT_OF_MONTH),
        (Pattern::LastLongUnitOfYear, PATTERN_LAST_LONG_UNIT_OF_YEAR),
        (Pattern::LastLongUnitOfMonthYear, PATTERN_LAST_LONG_UNIT_OF_MONTH_YEAR),
        (Pattern::LastLongUnitOfThisLongUnit, PATTERN_LAST_LONG_UNIT_OF_THIS_LONG_UNIT),
        (Pattern::LastLongUnitOfPrevLongUnit, PATTERN_LAST_LONG_UNIT_OF_PREV_LONG_UNIT),
        (Pattern::LastLongUnitOfPrevLongUnit, PATTERN_LAST_LONG_UNIT_OF_LAST_LONG_UNIT),
        (Pattern::LastLongUnitOfNextLongUnit, PATTERN_LAST_LONG_UNIT_OF_NEXT_LONG_UNIT),
        (Pattern::Timestamp, PATTERN_TIMESTAMP),
        (Pattern::TimestampFloat, PATTERN_TIMESTAMP_FLOAT),
        (Pattern::Year, PATTERN_YEAR),
        (Pattern::YearWeek, PATTERN_YW),
        (Pattern::YearWeek, PATTERN_YEAR_WEEK),
        (Pattern::DateYmd, PATTERN_DATE_YMD),
        (Pattern::DateYmd, PATTERN_DATE_YEAR_MONTH_DAY_DASHED),
        (Pattern::DateDmy, PATTERN_DATE_DMY),
        (Pattern::DateMdy, PATTERN_DATE_MDY),
        (Pattern::DateMonthDay, PATTERN_DATE_MONTH_DAY),
        (Pattern::DateMonthDayYear, PATTERN_DATE_MONTH_DAY_YEAR),
        (Pattern::DateMonthDayYear, PATTERN_DATE_MONTH_DAY_YEAR_DASHED),
        (Pattern::DateMonthNth, PATTERN_DATE_MONTH_NTH),
        (Pattern::DateMonthNthYear, PATTERN_DATE_MONTH_NTH_YEAR),
        (Pattern::DateDayMonth, PATTERN_DATE_DAY_MONTH),
        (Pattern::DateDayMonthYear, PATTERN_DATE_DAY_MONTH_YEAR),
        (Pattern::DateDayMonthYear, PATTERN_DATE_DAY_MONTH_YEAR_DASHED),
        (Pattern::DateDayMonth, PATTERN_DATE_NTH_MONTH),
        (Pattern::DateDayMonthYear, PATTERN_DATE_NTH_MONTH_YEAR),
        (Pattern::DateDayMonth, PATTERN_DATE_NTH_OF_MONTH),
        (Pattern::DateDayMonthYear, PATTERN_DATE_NTH_OF_MONTH_YEAR),
        (Pattern::DateTimeYmdHms, PATTERN_DATETIME_YMD_HMS),
        (Pattern::DateTimeYmdHms, PATTERN_DATETIME_ISO_YMD_HMS),
        (Pattern::DateTimeYmdHmsMs, PATTERN_DATETIME_YMD_HMS_MS),
        (Pattern::DateTimeYmdHmsMs, PATTERN_DATETIME_ISO_YMD_HMS_MS),
        (Pattern::DateWdayDayMonth, PATTERN_DATE_WDAY_DAY_MONTH),
        (Pattern::DateWdayDayMonthYear, PATTERN_DATE_WDAY_DAY_MONTH_YEAR),
        (Pattern::DateWdayDayMonth, PATTERN_DATE_WDAY_NTH_MONTH),
        (Pattern::DateWdayDayMonthYear, PATTERN_DATE_WDAY_NTH_MONTH_YEAR),
        (Pattern::DateWdayDayMonth, PATTERN_DATE_WDAY_NTH_OF_MONTH),
        (Pattern::DateWdayDayMonthYear, PATTERN_DATE_WDAY_NTH_OF_MONTH_YEAR),
        (Pattern::DateWdayMontDay, PATTERN_DATE_WDAY_MONTH_DAY),
        (Pattern::DateWdayMontDay, PATTERN_DATE_WDAY_MONTH_NTH),
        (Pattern::DateWdayMontDayYear, PATTERN_DATE_WDAY_MONTH_NTH_YEAR),
        (Pattern::DateWdayMontDayYear, PATTERN_DATE_WDAY_MONTH_YEAR),
        (Pattern::TimeHm, PATTERN_TIME_HM),
        (Pattern::TimeHm, PATTERN_TIME_AT_HM),
        (Pattern::TimeHm, PATTERN_TIME_AT_SIGN_HM),
        (Pattern::TimeHms, PATTERN_TIME_HMS),
        (Pattern::TimeHms, PATTERN_TIME_AT_HMS),
        (Pattern::TimeHms, PATTERN_TIME_AT_SIGN_HMS),
        (Pattern::TimeHmsMs, PATTERN_TIME_HMS_MS),
        (Pattern::TimeHmsMs, PATTERN_TIME_AT_HMS_MS),
        (Pattern::TimeHmsMs, PATTERN_TIME_AT_SIGN_HMS_MS),
        (Pattern::TimeMeridiemH, PATTERN_TIME_12H_HOUR),
        (Pattern::TimeMeridiemH, PATTERN_TIME_AT_12H_HOUR),
        (Pattern::TimeMeridiemH, PATTERN_TIME_AT_SIGN_12H_HOUR),
        (Pattern::TimeMeridiemH, PATTERN_TIME_12H_H),
        (Pattern::TimeMeridiemH, PATTERN_TIME_AT_12H_H),
        (Pattern::TimeMeridiemH, PATTERN_TIME_AT_SIGN_12H_H),
        (Pattern::TimeMeridiemHm, PATTERN_TIME_12H_HM),
        (Pattern::TimeMeridiemHm, PATTERN_TIME_AT_12H_HM),
        (Pattern::TimeMeridiemHm, PATTERN_TIME_AT_SIGN_12H_HM),
    ]
}
