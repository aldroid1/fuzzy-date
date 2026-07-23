mod python;

use chrono::{DateTime, FixedOffset, NaiveDate};
use fuzzy_date_rs::FuzzyDuration;
use fuzzy_date_rs::token::Token;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyDate, PyDateTime};
use std::collections::HashMap;

#[pymodule(gil_used = false)]
mod fuzzydate {
    use super::*;
    use fuzzy_date_rs::token::{Token, UnitGroup, UnitNames, WeekStartDay};
    use fuzzy_date_rs::{FuzzyDate as FuzzyDateRs, FuzzySeconds};

    #[pyclass]
    pub struct FuzzyDate {
        first_weekday: WeekStartDay,

        #[pyo3(get)]
        pub(crate) patterns: HashMap<String, String>,

        #[pyo3(get)]
        pub(crate) tokens: HashMap<String, u32>,

        #[pyo3(get, set)]
        pub(crate) units: HashMap<String, String>,

        #[pyo3(get, set)]
        pub(crate) units_long: HashMap<String, String>,

        #[pyo3(get, set)]
        pub(crate) units_short: HashMap<String, String>,
    }

    #[pymethods]
    impl FuzzyDate {
        #[new]
        pub fn new() -> Self {
            Self {
                first_weekday: WeekStartDay::Monday,
                patterns: HashMap::new(),
                tokens: HashMap::new(),
                units: UnitNames::get_defaults(&UnitGroup::Default),
                units_long: UnitNames::get_defaults(&UnitGroup::Long),
                units_short: UnitNames::get_defaults(&UnitGroup::Short),
            }
        }

        /// Add custom patterns that should replace default patterns, e.g.
        /// in order to localize English wording
        ///
        /// All strings are lowercased by default and merged with any previously
        /// added patterns. Colliding patterns will be replaced silently. Raises
        /// a ValueError if an unsupported pattern value is used, or if different
        /// amount of variables are used in the custom pattern.
        ///
        /// :param patterns: Map of patterns where keys are new patterns to identify and values
        ///                  are existing patterns to interpret them as. See
        ///                  fuzzydate.pattern.* constants for accepted values.
        /// :type source: dict[str, str]
        /// :raises ValueError
        /// :rtype Self
        ///
        #[pyo3(
            signature=(patterns,),
            text_signature = "(patterns: dict[str, Pattern])")
        ]
        pub fn add_patterns(mut slf: PyRefMut<Self>, patterns: HashMap<String, String>) -> PyResult<PyRefMut<Self>> {
            for (pattern, value) in patterns {
                if !fuzzy_date_rs::pattern::Pattern::is_valid(&value) {
                    return Err(PyValueError::new_err(format!(
                        "Pattern \"{}\" value \"{}\" does not exist",
                        pattern, value,
                    )));
                }

                let vars_in_custom: usize = pattern.split("[").count() - 1;
                let vars_in_value: usize = value.split("[").count() - 1;

                if vars_in_custom != vars_in_value {
                    return Err(PyValueError::new_err(format!(
                        "Pattern \"{}\" and \"{}\" have different variables",
                        pattern, value,
                    )));
                }

                slf.patterns.insert(pattern.to_lowercase(), value);
            }

            Ok(slf)
        }

        /// Add text strings to identify as tokens
        ///
        /// All strings are lowercased by default and merged with any previously
        /// added tokens. Overlapping keys will be replaced. Raises a ValueError
        /// if an unsupported token value is used.
        ///
        /// :param tokens: Map of tokens where keys are new strings to identify and values are
        ///                token values to classify them as. See fuzzydate.token.* constants
        ///                for accepted values.
        /// :type source: dict[str, int]
        /// :raises ValueError
        /// :rtype Self
        ///
        #[pyo3(
            signature=(tokens,),
            text_signature = "(tokens: dict[str, int])")
        ]
        pub fn add_tokens(mut slf: PyRefMut<Self>, tokens: HashMap<String, u32>) -> PyResult<PyRefMut<Self>> {
            for (keyword, gid) in tokens {
                if Token::from_gid(gid).is_some() {
                    slf.tokens.insert(keyword.to_lowercase(), gid);
                    continue;
                }

                return Err(PyValueError::new_err(format!("Token \"{}\" value {} does not exist", keyword, gid,)));
            }

            Ok(slf)
        }

        /// Change first weekday from Monday to Sunday
        ///
        /// :param use_sunday: True for Sunday, False for Monday
        /// :type use_sunday: bool
        /// :rtype Self
        ///
        pub fn set_first_weekday_sunday(mut slf: PyRefMut<Self>, use_sunday: bool) -> PyRefMut<Self> {
            slf.first_weekday = match use_sunday {
                true => WeekStartDay::Sunday,
                false => WeekStartDay::Monday,
            };
            slf
        }

        /// Turn time string into datetime.date object
        ///
        /// Current date (`today`) defaults to system date in UTC. Time of day
        /// is assumed to be midnight in case of any time adjustments. Raises
        /// a ValueError if the conversion fails.
        ///
        /// :param source: Source string
        /// :type source: str
        /// :param today: Current date. Defaults to system date in UTC.
        /// :type today: datetime.date, optional
        /// :raises ValueError
        /// :rtype datetime.date
        ///
        #[pyo3(
            signature = (source, today=None),
            text_signature = "(source: str, today: datetime.date = None)"
        )]
        pub fn to_date(&self, source: &str, today: Option<Bound<PyDate>>) -> PyResult<NaiveDate> {
            let timestamp = python::into_date(today)?;

            let result = FuzzyDateRs::from_time(timestamp)
                .set_first_weekday(self.first_weekday.to_owned())
                .set_custom_patterns(self.patterns.to_owned())
                .set_custom_tokens(tokens_from_gids(&self.tokens))
                .to_datetime(source);

            if let Some(v) = result {
                return Ok(v.date_naive());
            }

            Err(PyValueError::new_err(format!("Unable to convert \"{}\" into date", source)))
        }

        /// Turn time string into datetime.datetime object
        ///
        /// Current time (`now`) defaults to system time in UTC. If custom `now`
        /// does not contain a timezone, UTC timezone will be used. Raises a
        /// ValueError if the conversion fails.
        ///
        /// :param source: Source string
        /// :type source: str
        /// :param now: Current time. Defaults to system time in UTC.
        /// :type now: datetime.datetime, optional
        /// :raises ValueError
        /// :rtype datetime.datetime
        ///
        #[pyo3(
            signature = (source, now=None),
            text_signature = "(source: str, now: datetime.datetime = None)"
        )]
        pub fn to_datetime(&self, source: &str, now: Option<Bound<PyDateTime>>) -> PyResult<DateTime<FixedOffset>> {
            let timestamp = python::into_datetime(now)?;

            let result = FuzzyDateRs::from_time(timestamp)
                .set_first_weekday(self.first_weekday.to_owned())
                .set_custom_patterns(self.patterns.to_owned())
                .set_custom_tokens(tokens_from_gids(&self.tokens))
                .to_datetime(source);

            if let Some(v) = result {
                return Ok(v);
            }

            Err(PyValueError::new_err(format!("Unable to convert \"{}\" into datetime", source)))
        }

        /// Convert number of seconds into a time duration string
        ///
        /// Build a time duration string from number of seconds, e.g. 93600.0 is
        /// converted to "1d 2h". Maximum supported unit is weeks, minimum supported
        /// unit is seconds. Units that have no value (are 0) are not shown.
        ///
        /// Returns an empty string if number of seconds is not enough for the
        /// lowest shown unit.
        ///
        /// :param source: Number of seconds
        /// :type source: float
        /// :param unit: Unit type to use. Possible values are "long", "short" and None. Defaults to
        ///              None. For example, "long" would display seconds as "seconds", short as "s" and
        ///              default as "sec".
        /// :type unit: str, optional
        /// :param max: Maximum unit to show, defaults 'w' for weeks. Possible values are "s/sec" for
        ///             seconds, "min/mins" for minutes, "h/hr/hrs" for hours, "d/day/days" for days
        ///             and "w/week/weeks" for weeks.
        /// :type max: str, optional, default "w"
        /// :param min: Minimum unit to show, defaults 's' for seconds. Possible values are "s/sec" for
        ///             seconds, "min/mins" for minutes, "h/hr/hrs" for hours, "d/day/days" for days
        ///             and "w/week/weeks" for weeks.
        /// :type min: str, optional, default "s"
        /// :rtype str
        ///
        #[pyo3(
            signature = (seconds, units=None, max="w", min="s"),
            text_signature = "(seconds: float, units: Literal['long', 'short'] | None = None, max: Literal['s','sec','min','mins','h','hr','hrs','d','day','days','w','week','weeks'] = 'w', min: Literal['s','sec','min','mins','h','hr','hrs','d','day','days','w','week','weeks'] = 's')"
        )]
        fn to_duration(&self, seconds: f64, units: Option<&str>, max: &str, min: &str) -> PyResult<String> {
            let unit_group = units.unwrap_or("");

            let custom_units = match unit_group {
                "short" => self.units_short.to_owned(),
                "long" => self.units_long.to_owned(),
                _ => self.units.to_owned(),
            };

            let result = FuzzyDuration::new()
                .set_default_units(UnitGroup::from_str(unit_group))
                .set_custom_units(custom_units)
                .set_min_unit(min)
                .set_max_unit(max)
                .to_duration(seconds);

            Ok(result)
        }

        /// Turn time duration string into seconds
        ///
        /// Only accepts exact time duration strings, such as "1h" rather than
        /// "1 hour ago". Raises a ValueError if anything else than an exact
        /// length of time is provided, or if years or months have been included.
        ///
        /// :param source: Source string
        /// :type source: str
        /// :raises ValueError
        /// :rtype float
        ///
        #[pyo3(
            signature = (source,),
            text_signature = "(source: str)"
        )]
        fn to_seconds(&self, source: &str) -> PyResult<f64> {
            let config_patterns = self.patterns.to_owned();
            let config_tokens = tokens_from_gids(&self.tokens);

            let result = FuzzySeconds::new()
                .set_custom_patterns(config_patterns)
                .set_custom_tokens(config_tokens)
                .to_seconds(source);

            match result {
                Ok(v) => Ok(v),
                Err(e) => Err(PyValueError::new_err(e)),
            }
        }
    }

    #[pyclass(name = "pattern")]
    pub(crate) struct Patterns {}

    #[pymethods]
    impl Patterns {
        #[classattr]
        const NOW: &'static str = fuzzy_date_rs::pattern::PATTERN_NOW;
        #[classattr]
        const TODAY: &'static str = fuzzy_date_rs::pattern::PATTERN_TODAY;
        #[classattr]
        const MIDNIGHT: &'static str = fuzzy_date_rs::pattern::PATTERN_MIDNIGHT;
        #[classattr]
        const YESTERDAY: &'static str = fuzzy_date_rs::pattern::PATTERN_YESTERDAY;
        #[classattr]
        const TOMORROW: &'static str = fuzzy_date_rs::pattern::PATTERN_TOMORROW;

        #[classattr]
        const THIS_WDAY: &'static str = fuzzy_date_rs::pattern::PATTERN_THIS_WDAY;
        #[classattr]
        const PREV_WDAY: &'static str = fuzzy_date_rs::pattern::PATTERN_PREV_WDAY;
        #[classattr]
        const NEXT_WDAY: &'static str = fuzzy_date_rs::pattern::PATTERN_NEXT_WDAY;

        #[classattr]
        const THIS_MONTH: &'static str = fuzzy_date_rs::pattern::PATTERN_THIS_MONTH;
        #[classattr]
        const PREV_MONTH: &'static str = fuzzy_date_rs::pattern::PATTERN_PREV_MONTH;
        #[classattr]
        const NEXT_MONTH: &'static str = fuzzy_date_rs::pattern::PATTERN_NEXT_MONTH;

        #[classattr]
        const THIS_LONG_UNIT: &'static str = fuzzy_date_rs::pattern::PATTERN_THIS_LONG_UNIT;
        #[classattr]
        const PAST_LONG_UNIT: &'static str = fuzzy_date_rs::pattern::PATTERN_PAST_LONG_UNIT;
        #[classattr]
        const PREV_LONG_UNIT: &'static str = fuzzy_date_rs::pattern::PATTERN_PREV_LONG_UNIT;
        #[classattr]
        const NEXT_LONG_UNIT: &'static str = fuzzy_date_rs::pattern::PATTERN_NEXT_LONG_UNIT;

        #[classattr]
        const MINUS_UNIT: &'static str = fuzzy_date_rs::pattern::PATTERN_MINUS_UNIT;
        #[classattr]
        const MINUS_SHORT_UNIT: &'static str = fuzzy_date_rs::pattern::PATTERN_MINUS_SHORT_UNIT;
        #[classattr]
        const MINUS_LONG_UNIT: &'static str = fuzzy_date_rs::pattern::PATTERN_MINUS_LONG_UNIT;

        #[classattr]
        const PREV_N_LONG_UNIT: &'static str = fuzzy_date_rs::pattern::PATTERN_PREV_N_LONG_UNIT;

        #[classattr]
        const PLUS_UNIT: &'static str = fuzzy_date_rs::pattern::PATTERN_PLUS_UNIT;
        #[classattr]
        const PLUS_SHORT_UNIT: &'static str = fuzzy_date_rs::pattern::PATTERN_PLUS_SHORT_UNIT;
        #[classattr]
        const PLUS_LONG_UNIT: &'static str = fuzzy_date_rs::pattern::PATTERN_PLUS_LONG_UNIT;
        #[classattr]
        const UNIT_AGO: &'static str = fuzzy_date_rs::pattern::PATTERN_UNIT_AGO;
        #[classattr]
        const LONG_UNIT_AGO: &'static str = fuzzy_date_rs::pattern::PATTERN_LONG_UNIT_AGO;

        #[classattr]
        const FIRST_LONG_UNIT_OF_MONTH: &'static str = fuzzy_date_rs::pattern::PATTERN_FIRST_LONG_UNIT_OF_MONTH;
        #[classattr]
        const FIRST_LONG_UNIT_OF_MONTH_YEAR: &'static str =
            fuzzy_date_rs::pattern::PATTERN_FIRST_LONG_UNIT_OF_MONTH_YEAR;
        #[classattr]
        const FIRST_LONG_UNIT_OF_YEAR: &'static str = fuzzy_date_rs::pattern::PATTERN_FIRST_LONG_UNIT_OF_YEAR;
        #[classattr]
        const LAST_LONG_UNIT_OF_MONTH: &'static str = fuzzy_date_rs::pattern::PATTERN_LAST_LONG_UNIT_OF_MONTH;
        #[classattr]
        const LAST_LONG_UNIT_OF_MONTH_YEAR: &'static str = fuzzy_date_rs::pattern::PATTERN_LAST_LONG_UNIT_OF_MONTH_YEAR;
        #[classattr]
        const LAST_LONG_UNIT_OF_YEAR: &'static str = fuzzy_date_rs::pattern::PATTERN_LAST_LONG_UNIT_OF_YEAR;

        #[classattr]
        const FIRST_LONG_UNIT_OF_THIS_LONG_UNIT: &'static str =
            fuzzy_date_rs::pattern::PATTERN_FIRST_LONG_UNIT_OF_THIS_LONG_UNIT;
        #[classattr]
        const LAST_LONG_UNIT_OF_THIS_LONG_UNIT: &'static str =
            fuzzy_date_rs::pattern::PATTERN_LAST_LONG_UNIT_OF_THIS_LONG_UNIT;
        #[classattr]
        const FIRST_LONG_UNIT_OF_PREV_LONG_UNIT: &'static str =
            fuzzy_date_rs::pattern::PATTERN_FIRST_LONG_UNIT_OF_PREV_LONG_UNIT;
        #[classattr]
        const LAST_LONG_UNIT_OF_PREV_LONG_UNIT: &'static str =
            fuzzy_date_rs::pattern::PATTERN_LAST_LONG_UNIT_OF_PREV_LONG_UNIT;
        #[classattr]
        const FIRST_LONG_UNIT_OF_NEXT_LONG_UNIT: &'static str =
            fuzzy_date_rs::pattern::PATTERN_FIRST_LONG_UNIT_OF_NEXT_LONG_UNIT;
        #[classattr]
        const LAST_LONG_UNIT_OF_NEXT_LONG_UNIT: &'static str =
            fuzzy_date_rs::pattern::PATTERN_LAST_LONG_UNIT_OF_NEXT_LONG_UNIT;

        #[classattr]
        const FIRST_WDAY_OF_MONTH: &'static str = fuzzy_date_rs::pattern::PATTERN_FIRST_WDAY_OF_MONTH;
        #[classattr]
        const FIRST_WDAY_OF_MONTH_YEAR: &'static str = fuzzy_date_rs::pattern::PATTERN_FIRST_WDAY_OF_MONTH_YEAR;
        #[classattr]
        const FIRST_WDAY_OF_YEAR: &'static str = fuzzy_date_rs::pattern::PATTERN_FIRST_WDAY_OF_YEAR;
        #[classattr]
        const LAST_WDAY_OF_MONTH: &'static str = fuzzy_date_rs::pattern::PATTERN_LAST_WDAY_OF_MONTH;
        #[classattr]
        const LAST_WDAY_OF_MONTH_YEAR: &'static str = fuzzy_date_rs::pattern::PATTERN_LAST_WDAY_OF_MONTH_YEAR;
        #[classattr]
        const LAST_WDAY_OF_YEAR: &'static str = fuzzy_date_rs::pattern::PATTERN_LAST_WDAY_OF_YEAR;

        #[classattr]
        const TIMESTAMP: &'static str = fuzzy_date_rs::pattern::PATTERN_TIMESTAMP;
        #[classattr]
        const TIMESTAMP_FLOAT: &'static str = fuzzy_date_rs::pattern::PATTERN_TIMESTAMP_FLOAT;

        #[classattr]
        const DATE_YMD: &'static str = fuzzy_date_rs::pattern::PATTERN_DATE_YMD;
        #[classattr]
        const DATE_DMY: &'static str = fuzzy_date_rs::pattern::PATTERN_DATE_DMY;
        #[classattr]
        const DATE_MDY: &'static str = fuzzy_date_rs::pattern::PATTERN_DATE_MDY;

        #[classattr]
        const DATE_MONTH_DAY: &'static str = fuzzy_date_rs::pattern::PATTERN_DATE_MONTH_DAY;
        #[classattr]
        const DATE_MONTH_DAY_YEAR: &'static str = fuzzy_date_rs::pattern::PATTERN_DATE_MONTH_DAY_YEAR;
        #[classattr]
        const DATE_MONTH_NTH: &'static str = fuzzy_date_rs::pattern::PATTERN_DATE_MONTH_NTH;
        #[classattr]
        const DATE_MONTH_NTH_YEAR: &'static str = fuzzy_date_rs::pattern::PATTERN_DATE_MONTH_NTH_YEAR;
        #[classattr]
        const DATE_DAY_MONTH: &'static str = fuzzy_date_rs::pattern::PATTERN_DATE_DAY_MONTH;
        #[classattr]
        const DATE_DAY_MONTH_YEAR: &'static str = fuzzy_date_rs::pattern::PATTERN_DATE_DAY_MONTH_YEAR;
        #[classattr]
        const DATE_NTH_MONTH: &'static str = fuzzy_date_rs::pattern::PATTERN_DATE_NTH_MONTH;
        #[classattr]
        const DATE_NTH_MONTH_YEAR: &'static str = fuzzy_date_rs::pattern::PATTERN_DATE_NTH_MONTH_YEAR;

        #[classattr]
        const DATETIME_YMD_HMS: &'static str = fuzzy_date_rs::pattern::PATTERN_DATETIME_YMD_HMS;
        #[classattr]
        const DATETIME_YMD_HMS_MS: &'static str = fuzzy_date_rs::pattern::PATTERN_DATETIME_YMD_HMS_MS;

        #[classattr]
        const TIME_12H_H: &'static str = fuzzy_date_rs::pattern::PATTERN_TIME_12H_H;
        #[classattr]
        const TIME_12H_HM: &'static str = fuzzy_date_rs::pattern::PATTERN_TIME_12H_HM;
    }

    #[pyclass(name = "token")]
    pub(crate) struct Tokens {}

    #[pymethods]
    impl Tokens {
        // Ignore
        #[classattr]
        const IGNORE: i16 = fuzzy_date_rs::pattern::TOKEN_IGNORE;

        // Weekdays
        #[classattr]
        const WDAY_MON: i16 = fuzzy_date_rs::pattern::TOKEN_WDAY_MON;
        #[classattr]
        const WDAY_TUE: i16 = fuzzy_date_rs::pattern::TOKEN_WDAY_TUE;
        #[classattr]
        const WDAY_WED: i16 = fuzzy_date_rs::pattern::TOKEN_WDAY_WED;
        #[classattr]
        const WDAY_THU: i16 = fuzzy_date_rs::pattern::TOKEN_WDAY_THU;
        #[classattr]
        const WDAY_FRI: i16 = fuzzy_date_rs::pattern::TOKEN_WDAY_FRI;
        #[classattr]
        const WDAY_SAT: i16 = fuzzy_date_rs::pattern::TOKEN_WDAY_SAT;
        #[classattr]
        const WDAY_SUN: i16 = fuzzy_date_rs::pattern::TOKEN_WDAY_SUN;

        // Months
        #[classattr]
        const MONTH_JAN: i16 = fuzzy_date_rs::pattern::TOKEN_MONTH_JAN;
        #[classattr]
        const MONTH_FEB: i16 = fuzzy_date_rs::pattern::TOKEN_MONTH_FEB;
        #[classattr]
        const MONTH_MAR: i16 = fuzzy_date_rs::pattern::TOKEN_MONTH_MAR;
        #[classattr]
        const MONTH_APR: i16 = fuzzy_date_rs::pattern::TOKEN_MONTH_APR;
        #[classattr]
        const MONTH_MAY: i16 = fuzzy_date_rs::pattern::TOKEN_MONTH_MAY;
        #[classattr]
        const MONTH_JUN: i16 = fuzzy_date_rs::pattern::TOKEN_MONTH_JUN;
        #[classattr]
        const MONTH_JUL: i16 = fuzzy_date_rs::pattern::TOKEN_MONTH_JUL;
        #[classattr]
        const MONTH_AUG: i16 = fuzzy_date_rs::pattern::TOKEN_MONTH_AUG;
        #[classattr]
        const MONTH_SEP: i16 = fuzzy_date_rs::pattern::TOKEN_MONTH_SEP;
        #[classattr]
        const MONTH_OCT: i16 = fuzzy_date_rs::pattern::TOKEN_MONTH_OCT;
        #[classattr]
        const MONTH_NOV: i16 = fuzzy_date_rs::pattern::TOKEN_MONTH_NOV;
        #[classattr]
        const MONTH_DEC: i16 = fuzzy_date_rs::pattern::TOKEN_MONTH_DEC;

        #[classattr]
        const UNIT_SEC: i16 = fuzzy_date_rs::pattern::TOKEN_UNIT_SEC;
        #[classattr]
        const UNIT_MIN: i16 = fuzzy_date_rs::pattern::TOKEN_UNIT_MIN;
        #[classattr]
        const UNIT_HRS: i16 = fuzzy_date_rs::pattern::TOKEN_UNIT_HRS;

        #[classattr]
        const SHORT_UNIT_SEC: i16 = fuzzy_date_rs::pattern::TOKEN_SHORT_UNIT_SEC;
        #[classattr]
        const SHORT_UNIT_HRS: i16 = fuzzy_date_rs::pattern::TOKEN_SHORT_UNIT_HRS;
        #[classattr]
        const SHORT_UNIT_DAY: i16 = fuzzy_date_rs::pattern::TOKEN_SHORT_UNIT_DAY;
        #[classattr]
        const SHORT_UNIT_WEEK: i16 = fuzzy_date_rs::pattern::TOKEN_SHORT_UNIT_WEEK;
        #[classattr]
        const SHORT_UNIT_MONTH: i16 = fuzzy_date_rs::pattern::TOKEN_SHORT_UNIT_MONTH;
        #[classattr]
        const SHORT_UNIT_YEAR: i16 = fuzzy_date_rs::pattern::TOKEN_SHORT_UNIT_YEAR;

        #[classattr]
        const LONG_UNIT_SEC: i16 = fuzzy_date_rs::pattern::TOKEN_LONG_UNIT_SEC;
        #[classattr]
        const LONG_UNIT_MIN: i16 = fuzzy_date_rs::pattern::TOKEN_LONG_UNIT_MIN;
        #[classattr]
        const LONG_UNIT_HRS: i16 = fuzzy_date_rs::pattern::TOKEN_LONG_UNIT_HRS;
        #[classattr]
        const LONG_UNIT_DAY: i16 = fuzzy_date_rs::pattern::TOKEN_LONG_UNIT_DAY;
        #[classattr]
        const LONG_UNIT_WEEK: i16 = fuzzy_date_rs::pattern::TOKEN_LONG_UNIT_WEEK;
        #[classattr]
        const LONG_UNIT_MONTH: i16 = fuzzy_date_rs::pattern::TOKEN_LONG_UNIT_MONTH;
        #[classattr]
        const LONG_UNIT_YEAR: i16 = fuzzy_date_rs::pattern::TOKEN_LONG_UNIT_YEAR;

        #[classattr]
        const MERIDIEM_AM: i16 = fuzzy_date_rs::pattern::TOKEN_MERIDIEM_AM;
        #[classattr]
        const MERIDIEM_PM: i16 = fuzzy_date_rs::pattern::TOKEN_MERIDIEM_PM;
    }

    #[pyclass(name = "unit")]
    pub(crate) struct Units {}

    #[pymethods]
    impl Units {
        #[classattr]
        const DAY: &'static str = fuzzy_date_rs::pattern::UNIT_DAY;
        #[classattr]
        const DAYS: &'static str = fuzzy_date_rs::pattern::UNIT_DAYS;
        #[classattr]
        const HOUR: &'static str = fuzzy_date_rs::pattern::UNIT_HOUR;
        #[classattr]
        const HOURS: &'static str = fuzzy_date_rs::pattern::UNIT_HOURS;
        #[classattr]
        const MINUTE: &'static str = fuzzy_date_rs::pattern::UNIT_MINUTE;
        #[classattr]
        const MINUTES: &'static str = fuzzy_date_rs::pattern::UNIT_MINUTES;
        #[classattr]
        const SECOND: &'static str = fuzzy_date_rs::pattern::UNIT_SECOND;
        #[classattr]
        const SECONDS: &'static str = fuzzy_date_rs::pattern::UNIT_SECONDS;
        #[classattr]
        const WEEK: &'static str = fuzzy_date_rs::pattern::UNIT_WEEK;
        #[classattr]
        const WEEKS: &'static str = fuzzy_date_rs::pattern::UNIT_WEEKS;
    }

    /// Turn time string into datetime.date object
    ///
    /// Current date (`today`) defaults to system date in UTC. Time of day
    /// is assumed to be midnight in case of any time adjustments. Raises
    /// a ValueError if the conversion fails.
    ///
    /// :param source: Source string
    /// :type source: str
    /// :param today: Current date. Defaults to system date in UTC.
    /// :type today: datetime.date, optional
    /// :param weekday_start_mon: Whether weeks begin on Monday instead of Sunday. Defaults to True.
    /// :type weekday_start_mon: bool, optional, default True
    /// :raises ValueError
    /// :rtype datetime.date
    ///
    #[pyfunction]
    #[pyo3(
        signature = (source, today=None, weekday_start_mon=true),
        text_signature = "(source: str, today: datetime.date = None, weekday_start_mon: bool = True)"
    )]
    fn to_date(
        py: Python<'_>,
        source: &str,
        today: Option<Bound<PyDate>>,
        weekday_start_mon: bool,
    ) -> PyResult<NaiveDate> {
        let fd = Py::new(py, FuzzyDate::new())?;
        let fd_ref = fd.borrow_mut(py);
        FuzzyDate::set_first_weekday_sunday(fd_ref, !weekday_start_mon).to_date(source, today)
    }

    /// Turn time string into datetime.datetime object
    ///
    /// Current time (`now`) defaults to system time in UTC. If custom `now`
    /// does not contain a timezone, UTC timezone will be used. Raises a
    /// ValueError if the conversion fails.
    ///
    /// :param source: Source string
    /// :type source: str
    /// :param now: Current time. Defaults to system time in UTC.
    /// :type now: datetime.datetime, optional
    /// :param weekday_start_mon: Whether weeks begin on Monday instead of Sunday. Defaults to True.
    /// :type weekday_start_mon: bool, optional, default True
    /// :raises ValueError
    /// :rtype datetime.datetime
    #[pyfunction]
    #[pyo3(
        signature = (source, now=None, weekday_start_mon=true),
        text_signature = "(source: str, today: datetime.date = None, weekday_start_mon: bool = True)"
    )]
    pub fn to_datetime(
        py: Python<'_>,
        source: &str,
        now: Option<Bound<PyDateTime>>,
        weekday_start_mon: bool,
    ) -> PyResult<DateTime<FixedOffset>> {
        let fd = Py::new(py, FuzzyDate::new())?;
        let fd_ref = fd.borrow_mut(py);
        FuzzyDate::set_first_weekday_sunday(fd_ref, !weekday_start_mon).to_datetime(source, now)
    }

    /// Convert number of seconds into a time duration string
    ///
    /// Build a time duration string from number of seconds, e.g. 93600.0 is
    /// converted to "1d 2h". Maximum supported unit is weeks, minimum supported
    /// unit is seconds. Units that have no value (are 0) are not shown.
    ///
    /// Returns an empty string if number of seconds is not enough for the
    /// lowest shown unit.
    ///
    /// :param source: Number of seconds
    /// :type source: float
    /// :param unit: Unit type to use. Possible values are "long", "short" and None. Defaults to
    ///              None. For example, "long" would display seconds as "seconds", short as "s" and
    ///              default as "sec".
    /// :type unit: str, optional
    /// :param max: Maximum unit to show, defaults 'w' for weeks. Possible values are "s/sec" for
    ///             seconds, "min/mins" for minutes, "h/hr/hrs" for hours, "d/day/days" for days
    ///             and "w/week/weeks" for weeks.
    /// :type max: str, optional, default "w"
    /// :param min: Minimum unit to show, defaults 's' for seconds. Possible values are "s/sec" for
    ///             seconds, "min/mins" for minutes, "h/hr/hrs" for hours, "d/day/days" for days
    ///             and "w/week/weeks" for weeks.
    /// :type min: str, optional, default "s"
    /// :rtype str
    ///
    #[pyfunction]
    #[pyo3(
            signature = (seconds, units=None, max="w", min="s"),
            text_signature = "(seconds: float, units: str = None, max: str = 'w', min: str = 's')"
    )]
    pub fn to_duration(seconds: f64, units: Option<&str>, max: &str, min: &str) -> PyResult<String> {
        FuzzyDate::new().to_duration(seconds, units, max, min)
    }

    /// Turn time duration string into seconds
    ///
    /// Only accepts exact time duration strings, such as "1h" rather than
    /// "1 hour ago". Raises a ValueError if anything else than an exact
    /// length of time is provided, or if years or months have been included.
    ///
    /// :param source: Source string
    /// :type source: str
    /// :raises ValueError
    /// :rtype float
    ///
    #[pyfunction]
    #[pyo3(
            signature = (source,),
            text_signature = "(source: str)"
    )]
    pub fn to_seconds(source: &str) -> PyResult<f64> {
        FuzzyDate::new().to_seconds(source)
    }

    #[pymodule_init]
    fn init(_module: &Bound<'_, PyModule>) -> PyResult<()> {
        Ok(())
    }
}

/// Turn custom GID tokens registered with Python into internal tokens
fn tokens_from_gids(tokens: &HashMap<String, u32>) -> HashMap<String, Token> {
    let mut result = HashMap::new();

    for (keyword, token_gid) in tokens {
        if let Some(token) = Token::from_gid(token_gid.to_owned()) {
            result.insert(keyword.to_owned(), token);
        }
    }

    result
}
