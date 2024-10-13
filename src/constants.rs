use std::collections::HashMap;

pub(crate) const PATTERN_NOW: &'static str = "now";
pub(crate) const PATTERN_TODAY: &'static str = "today";
pub(crate) const PATTERN_MIDNIGHT: &'static str = "midnight";
pub(crate) const PATTERN_YESTERDAY: &'static str = "yesterday";
pub(crate) const PATTERN_TOMORROW: &'static str = "tomorrow";

pub(crate) const PATTERN_THIS_WDAY: &'static str = "this [wday]";
pub(crate) const PATTERN_PREV_WDAY: &'static str = "prev [wday]";
pub(crate) const PATTERN_LAST_WDAY: &'static str = "last [wday]";
pub(crate) const PATTERN_NEXT_WDAY: &'static str = "next [wday]";

pub(crate) const PATTERN_THIS_LONG_UNIT: &'static str = "this [long_unit]";
pub(crate) const PATTERN_PREV_LONG_UNIT: &'static str = "prev [long_unit]";
pub(crate) const PATTERN_LAST_LONG_UNIT: &'static str = "last [long_unit]";
pub(crate) const PATTERN_NEXT_LONG_UNIT: &'static str = "next [long_unit]";

pub(crate) const PATTERN_MINUS_UNIT: &'static str = "-[int][unit]";
pub(crate) const PATTERN_MINUS_SHORT_UNIT: &'static str = "-[int][short_unit]";
pub(crate) const PATTERN_MINUS_LONG_UNIT: &'static str = "-[int] [long_unit]";

pub(crate) const PATTERN_PLUS_UNIT: &'static str = "+[int][unit]";
pub(crate) const PATTERN_PLUS_SHORT_UNIT: &'static str = "+[int][short_unit]";
pub(crate) const PATTERN_PLUS_LONG_UNIT: &'static str = "+[int] [long_unit]";
pub(crate) const PATTERN_UNIT_AGO: &'static str = "[int] [unit] ago";
pub(crate) const PATTERN_LONG_UNIT_AGO: &'static str = "[int] [long_unit] ago";

pub(crate) const PATTERN_FIRST_LONG_UNIT_OF_MONTH: &'static str = "first [long_unit] of [month]";
pub(crate) const PATTERN_LAST_LONG_UNIT_OF_MONTH: &'static str = "last [long_unit] of [month]";
pub(crate) const PATTERN_FIRST_LONG_UNIT_OF_THIS_LONG_UNIT: &'static str = "first [long_unit] of this [long_unit]";
pub(crate) const PATTERN_LAST_LONG_UNIT_OF_THIS_LONG_UNIT: &'static str = "last [long_unit] of this [long_unit]";
pub(crate) const PATTERN_FIRST_LONG_UNIT_OF_PREV_LONG_UNIT: &'static str = "first [long_unit] of prev [long_unit]";
pub(crate) const PATTERN_LAST_LONG_UNIT_OF_PREV_LONG_UNIT: &'static str = "last [long_unit] of prev [long_unit]";
pub(crate) const PATTERN_FIRST_LONG_UNIT_OF_LAST_LONG_UNIT: &'static str = "first [long_unit] of last [long_unit]";
pub(crate) const PATTERN_LAST_LONG_UNIT_OF_LAST_LONG_UNIT: &'static str = "last [long_unit] of last [long_unit]";
pub(crate) const PATTERN_FIRST_LONG_UNIT_OF_NEXT_LONG_UNIT: &'static str = "first [long_unit] of next [long_unit]";
pub(crate) const PATTERN_LAST_LONG_UNIT_OF_NEXT_LONG_UNIT: &'static str = "last [long_unit] of next [long_unit]";

pub(crate) const PATTERN_TIMESTAMP: &'static str = "[timestamp]";
pub(crate) const PATTERN_TIMESTAMP_FLOAT: &'static str = "[timestamp].[int]";

pub(crate) const PATTERN_DATE_YMD: &'static str = "[year]-[int]-[int]";
pub(crate) const PATTERN_DATE_DMY: &'static str = "[int].[int].[year]";
pub(crate) const PATTERN_DATE_MDY: &'static str = "[int]/[int]/[year]";

pub(crate) const PATTERN_DATE_MONTH_DAY_YEAR: &'static str = "[month] [int] [year]";
pub(crate) const PATTERN_DATE_MONTH_NTH_YEAR: &'static str = "[month] [nth] [year]";
pub(crate) const PATTERN_DATE_DAY_MONTH_YEAR: &'static str = "[int] [month] [year]";

pub(crate) const PATTERN_DATETIME_YMD_HM: &'static str = "[year]-[int]-[int] [int]:[int]";
pub(crate) const PATTERN_DATETIME_YMD_HMS: &'static str = "[year]-[int]-[int] [int]:[int]:[int]";

#[derive(PartialEq, Eq, Hash)]
pub(crate) enum Pattern {
    Now,
    Today,
    Midnight,
    Yesterday,
    Tomorrow,

    ThisWday,
    PrevWday,
    LastWday,
    NextWday,

    ThisLongUnit,
    PrevLongUnit,
    LastLongUnit,
    NextLongUnit,

    MinusUnit,
    MinusShortUnit,
    MinusLongUnit,

    PlusUnit,
    PlusShortUnit,
    PlusLongUnit,

    UnitAgo,
    LongUnitAgo,

    FirstLongUnitOfMonth,
    LastLongUnitOfMonth,

    FirstLongUnitOfThisLongUnit,
    LastLongUnitOfThisLongUnit,

    FirstLongUnitOfPrevLongUnit,
    LastLongUnitOfPrevLongUnit,

    FirstLongUnitOfLastLongUnit,
    LastLongUnitOfLastLongUnit,

    FirstLongUnitOfNextLongUnit,
    LastLongUnitOfNextLongUnit,

    Timestamp,
    TimestampFloat,

    DateYmd,
    DateDmy,
    DateMdy,
    DateMonthDayYear,
    DateMonthNthYear,
    DateDayMonthYear,
    DateTimeYmdHm,
    DateTimeYmdHms,
}

impl Pattern {
    pub(crate) fn value(key: &Pattern) -> &'static str {
        match patterns().get(key) {
            Some(v) => v,
            None => "",
        }
    }

    pub(crate) fn is_valid(value: &str) -> bool {
        patterns().values().find(|&&v| v == value).is_some()
    }
}

fn patterns() -> HashMap<Pattern, &'static str> {
    HashMap::from([
        (Pattern::Now, PATTERN_NOW),
        (Pattern::Today, PATTERN_TODAY),
        (Pattern::Midnight, PATTERN_MIDNIGHT),
        (Pattern::Yesterday, PATTERN_YESTERDAY),
        (Pattern::Tomorrow, PATTERN_TOMORROW),
        (Pattern::ThisWday, PATTERN_THIS_WDAY),
        (Pattern::PrevWday, PATTERN_PREV_WDAY),
        (Pattern::LastWday, PATTERN_LAST_WDAY),
        (Pattern::NextWday, PATTERN_NEXT_WDAY),
        (Pattern::ThisLongUnit, PATTERN_THIS_LONG_UNIT),
        (Pattern::PrevLongUnit, PATTERN_PREV_LONG_UNIT),
        (Pattern::LastLongUnit, PATTERN_LAST_LONG_UNIT),
        (Pattern::NextLongUnit, PATTERN_NEXT_LONG_UNIT),
        (Pattern::MinusUnit, PATTERN_MINUS_UNIT),
        (Pattern::MinusShortUnit, PATTERN_MINUS_SHORT_UNIT),
        (Pattern::MinusLongUnit, PATTERN_MINUS_LONG_UNIT),
        (Pattern::PlusUnit, PATTERN_PLUS_UNIT),
        (Pattern::PlusShortUnit, PATTERN_PLUS_SHORT_UNIT),
        (Pattern::PlusLongUnit, PATTERN_PLUS_LONG_UNIT),
        (Pattern::UnitAgo, PATTERN_UNIT_AGO),
        (Pattern::LongUnitAgo, PATTERN_LONG_UNIT_AGO),
        (Pattern::FirstLongUnitOfMonth, PATTERN_FIRST_LONG_UNIT_OF_MONTH),
        (Pattern::LastLongUnitOfMonth, PATTERN_LAST_LONG_UNIT_OF_MONTH),
        (Pattern::FirstLongUnitOfThisLongUnit, PATTERN_FIRST_LONG_UNIT_OF_THIS_LONG_UNIT),
        (Pattern::LastLongUnitOfThisLongUnit, PATTERN_LAST_LONG_UNIT_OF_THIS_LONG_UNIT),
        (Pattern::FirstLongUnitOfPrevLongUnit, PATTERN_FIRST_LONG_UNIT_OF_PREV_LONG_UNIT),
        (Pattern::LastLongUnitOfPrevLongUnit, PATTERN_LAST_LONG_UNIT_OF_PREV_LONG_UNIT),
        (Pattern::FirstLongUnitOfLastLongUnit, PATTERN_FIRST_LONG_UNIT_OF_LAST_LONG_UNIT),
        (Pattern::LastLongUnitOfLastLongUnit, PATTERN_LAST_LONG_UNIT_OF_LAST_LONG_UNIT),
        (Pattern::FirstLongUnitOfNextLongUnit, PATTERN_FIRST_LONG_UNIT_OF_NEXT_LONG_UNIT),
        (Pattern::LastLongUnitOfNextLongUnit, PATTERN_LAST_LONG_UNIT_OF_NEXT_LONG_UNIT),
        (Pattern::Timestamp, PATTERN_TIMESTAMP),
        (Pattern::TimestampFloat, PATTERN_TIMESTAMP_FLOAT),
        (Pattern::DateYmd, PATTERN_DATE_YMD),
        (Pattern::DateDmy, PATTERN_DATE_DMY),
        (Pattern::DateMdy, PATTERN_DATE_MDY),
        (Pattern::DateMonthDayYear, PATTERN_DATE_MONTH_DAY_YEAR),
        (Pattern::DateMonthNthYear, PATTERN_DATE_MONTH_NTH_YEAR),
        (Pattern::DateDayMonthYear, PATTERN_DATE_DAY_MONTH_YEAR),
        (Pattern::DateTimeYmdHm, PATTERN_DATETIME_YMD_HM),
        (Pattern::DateTimeYmdHms, PATTERN_DATETIME_YMD_HMS),
    ])
}