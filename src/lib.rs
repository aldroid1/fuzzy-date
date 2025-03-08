mod constants;
mod convert;
mod fuzzy;
mod python;
mod token;

use crate::fuzzy::UnitLocale;
use crate::token::{Token, TokenType};
use chrono::{DateTime, Duration, FixedOffset, NaiveDate, Utc};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyDate, PyDateTime};
use std::collections::HashMap;

#[pymodule]
mod fuzzydate {
    use super::*;
    use crate::fuzzydate::__core__::Config;

    const ATTR_CONFIG: &'static str = "config";

    #[pymodule]
    mod __core__ {
        use super::*;

        #[pyclass]
        pub(crate) struct Config {
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
        impl Config {
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
            /// :rtype None
            ///
            #[pyo3(text_signature = "(patterns: dict[str, str]) -> None")]
            fn add_patterns(&mut self, patterns: HashMap<String, String>) -> PyResult<()> {
                for (pattern, value) in patterns {
                    if !constants::Pattern::is_valid(&value) {
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

                    self.patterns.insert(pattern.to_lowercase(), value);
                }

                Ok(())
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
            /// :rtype None
            ///
            #[pyo3(text_signature = "(tokens: dict[str, int]) -> None")]
            fn add_tokens(&mut self, tokens: HashMap<String, u32>) -> PyResult<()> {
                for (keyword, gid) in tokens {
                    if gid_into_token(gid).is_some() {
                        self.tokens.insert(keyword.to_lowercase(), gid);
                        continue;
                    }

                    return Err(PyValueError::new_err(format!("Token \"{}\" value {} does not exist", keyword, gid,)));
                }

                Ok(())
            }
        }
    }

    #[pyclass(name = "pattern")]
    pub(crate) struct Patterns {}

    #[pymethods]
    impl Patterns {
        #[classattr]
        const NOW: &'static str = constants::PATTERN_NOW;
        #[classattr]
        const TODAY: &'static str = constants::PATTERN_TODAY;
        #[classattr]
        const MIDNIGHT: &'static str = constants::PATTERN_MIDNIGHT;
        #[classattr]
        const YESTERDAY: &'static str = constants::PATTERN_YESTERDAY;
        #[classattr]
        const TOMORROW: &'static str = constants::PATTERN_TOMORROW;

        #[classattr]
        const THIS_WDAY: &'static str = constants::PATTERN_THIS_WDAY;
        #[classattr]
        const PREV_WDAY: &'static str = constants::PATTERN_PREV_WDAY;
        #[classattr]
        #[deprecated]
        const LAST_WDAY: &'static str = constants::PATTERN_LAST_WDAY;
        #[classattr]
        const NEXT_WDAY: &'static str = constants::PATTERN_NEXT_WDAY;

        #[classattr]
        const THIS_MONTH: &'static str = constants::PATTERN_THIS_MONTH;
        #[classattr]
        const PREV_MONTH: &'static str = constants::PATTERN_PREV_MONTH;
        #[classattr]
        #[deprecated]
        const LAST_MONTH: &'static str = constants::PATTERN_LAST_MONTH;
        #[classattr]
        const NEXT_MONTH: &'static str = constants::PATTERN_NEXT_MONTH;

        #[classattr]
        const THIS_LONG_UNIT: &'static str = constants::PATTERN_THIS_LONG_UNIT;
        #[classattr]
        const PAST_LONG_UNIT: &'static str = constants::PATTERN_PAST_LONG_UNIT;
        #[classattr]
        const PREV_LONG_UNIT: &'static str = constants::PATTERN_PREV_LONG_UNIT;
        #[classattr]
        #[deprecated]
        const LAST_LONG_UNIT: &'static str = constants::PATTERN_LAST_LONG_UNIT;
        #[classattr]
        const NEXT_LONG_UNIT: &'static str = constants::PATTERN_NEXT_LONG_UNIT;

        #[classattr]
        const MINUS_UNIT: &'static str = constants::PATTERN_MINUS_UNIT;
        #[classattr]
        const MINUS_SHORT_UNIT: &'static str = constants::PATTERN_MINUS_SHORT_UNIT;
        #[classattr]
        const MINUS_LONG_UNIT: &'static str = constants::PATTERN_MINUS_LONG_UNIT;

        #[classattr]
        const PREV_N_LONG_UNIT: &'static str = constants::PATTERN_PREV_N_LONG_UNIT;

        #[classattr]
        const PLUS_UNIT: &'static str = constants::PATTERN_PLUS_UNIT;
        #[classattr]
        const PLUS_SHORT_UNIT: &'static str = constants::PATTERN_PLUS_SHORT_UNIT;
        #[classattr]
        const PLUS_LONG_UNIT: &'static str = constants::PATTERN_PLUS_LONG_UNIT;
        #[classattr]
        const UNIT_AGO: &'static str = constants::PATTERN_UNIT_AGO;
        #[classattr]
        const LONG_UNIT_AGO: &'static str = constants::PATTERN_LONG_UNIT_AGO;

        #[classattr]
        const FIRST_LONG_UNIT_OF_MONTH: &'static str = constants::PATTERN_FIRST_LONG_UNIT_OF_MONTH;
        #[classattr]
        const FIRST_LONG_UNIT_OF_MONTH_YEAR: &'static str = constants::PATTERN_FIRST_LONG_UNIT_OF_MONTH_YEAR;
        #[classattr]
        const FIRST_LONG_UNIT_OF_YEAR: &'static str = constants::PATTERN_FIRST_LONG_UNIT_OF_YEAR;
        #[classattr]
        const LAST_LONG_UNIT_OF_MONTH: &'static str = constants::PATTERN_LAST_LONG_UNIT_OF_MONTH;
        #[classattr]
        const LAST_LONG_UNIT_OF_MONTH_YEAR: &'static str = constants::PATTERN_LAST_LONG_UNIT_OF_MONTH_YEAR;
        #[classattr]
        const LAST_LONG_UNIT_OF_YEAR: &'static str = constants::PATTERN_LAST_LONG_UNIT_OF_YEAR;

        #[classattr]
        const FIRST_LONG_UNIT_OF_THIS_LONG_UNIT: &'static str = constants::PATTERN_FIRST_LONG_UNIT_OF_THIS_LONG_UNIT;
        #[classattr]
        const LAST_LONG_UNIT_OF_THIS_LONG_UNIT: &'static str = constants::PATTERN_LAST_LONG_UNIT_OF_THIS_LONG_UNIT;
        #[classattr]
        const FIRST_LONG_UNIT_OF_PREV_LONG_UNIT: &'static str = constants::PATTERN_FIRST_LONG_UNIT_OF_PREV_LONG_UNIT;
        #[classattr]
        const LAST_LONG_UNIT_OF_PREV_LONG_UNIT: &'static str = constants::PATTERN_LAST_LONG_UNIT_OF_PREV_LONG_UNIT;
        #[classattr]
        #[deprecated]
        const FIRST_LONG_UNIT_OF_LAST_LONG_UNIT: &'static str = constants::PATTERN_FIRST_LONG_UNIT_OF_LAST_LONG_UNIT;
        #[classattr]
        #[deprecated]
        const LAST_LONG_UNIT_OF_LAST_LONG_UNIT: &'static str = constants::PATTERN_LAST_LONG_UNIT_OF_LAST_LONG_UNIT;
        #[classattr]
        const FIRST_LONG_UNIT_OF_NEXT_LONG_UNIT: &'static str = constants::PATTERN_FIRST_LONG_UNIT_OF_NEXT_LONG_UNIT;
        #[classattr]
        const LAST_LONG_UNIT_OF_NEXT_LONG_UNIT: &'static str = constants::PATTERN_LAST_LONG_UNIT_OF_NEXT_LONG_UNIT;

        #[classattr]
        const FIRST_WDAY_OF_MONTH: &'static str = constants::PATTERN_FIRST_WDAY_OF_MONTH;
        #[classattr]
        const FIRST_WDAY_OF_MONTH_YEAR: &'static str = constants::PATTERN_FIRST_WDAY_OF_MONTH_YEAR;
        #[classattr]
        const FIRST_WDAY_OF_YEAR: &'static str = constants::PATTERN_FIRST_WDAY_OF_YEAR;
        #[classattr]
        const LAST_WDAY_OF_MONTH: &'static str = constants::PATTERN_LAST_WDAY_OF_MONTH;
        #[classattr]
        const LAST_WDAY_OF_MONTH_YEAR: &'static str = constants::PATTERN_LAST_WDAY_OF_MONTH_YEAR;
        #[classattr]
        const LAST_WDAY_OF_YEAR: &'static str = constants::PATTERN_LAST_WDAY_OF_YEAR;

        #[classattr]
        const TIMESTAMP: &'static str = constants::PATTERN_TIMESTAMP;
        #[classattr]
        const TIMESTAMP_FLOAT: &'static str = constants::PATTERN_TIMESTAMP_FLOAT;

        #[classattr]
        const DATE_YMD: &'static str = constants::PATTERN_DATE_YMD;
        #[classattr]
        const DATE_DMY: &'static str = constants::PATTERN_DATE_DMY;
        #[classattr]
        const DATE_MDY: &'static str = constants::PATTERN_DATE_MDY;

        #[classattr]
        const DATE_MONTH_DAY: &'static str = constants::PATTERN_DATE_MONTH_DAY;
        #[classattr]
        const DATE_MONTH_DAY_YEAR: &'static str = constants::PATTERN_DATE_MONTH_DAY_YEAR;
        #[classattr]
        const DATE_MONTH_NTH: &'static str = constants::PATTERN_DATE_MONTH_NTH;
        #[classattr]
        const DATE_MONTH_NTH_YEAR: &'static str = constants::PATTERN_DATE_MONTH_NTH_YEAR;
        #[classattr]
        const DATE_DAY_MONTH: &'static str = constants::PATTERN_DATE_DAY_MONTH;
        #[classattr]
        const DATE_DAY_MONTH_YEAR: &'static str = constants::PATTERN_DATE_DAY_MONTH_YEAR;
        #[classattr]
        const DATE_NTH_MONTH: &'static str = constants::PATTERN_DATE_NTH_MONTH;
        #[classattr]
        const DATE_NTH_MONTH_YEAR: &'static str = constants::PATTERN_DATE_NTH_MONTH_YEAR;

        #[classattr]
        #[deprecated]
        const DATETIME_YMD_HM: &'static str = "[year]-[int]-[int] [int]:[int]";
        #[classattr]
        const DATETIME_YMD_HMS: &'static str = constants::PATTERN_DATETIME_YMD_HMS;
        #[classattr]
        const DATETIME_YMD_HMS_MS: &'static str = constants::PATTERN_DATETIME_YMD_HMS_MS;

        #[classattr]
        const TIME_12H_H: &'static str = constants::PATTERN_TIME_12H_H;
        #[classattr]
        const TIME_12H_HM: &'static str = constants::PATTERN_TIME_12H_HM;
    }

    #[pyclass(name = "token")]
    pub(crate) struct Tokens {}

    #[pymethods]
    impl Tokens {
        // Weekdays
        #[classattr]
        const WDAY_MON: i16 = constants::TOKEN_WDAY_MON;
        #[classattr]
        const WDAY_TUE: i16 = constants::TOKEN_WDAY_TUE;
        #[classattr]
        const WDAY_WED: i16 = constants::TOKEN_WDAY_WED;
        #[classattr]
        const WDAY_THU: i16 = constants::TOKEN_WDAY_THU;
        #[classattr]
        const WDAY_FRI: i16 = constants::TOKEN_WDAY_FRI;
        #[classattr]
        const WDAY_SAT: i16 = constants::TOKEN_WDAY_SAT;
        #[classattr]
        const WDAY_SUN: i16 = constants::TOKEN_WDAY_SUN;

        // Months
        #[classattr]
        const MONTH_JAN: i16 = constants::TOKEN_MONTH_JAN;
        #[classattr]
        const MONTH_FEB: i16 = constants::TOKEN_MONTH_FEB;
        #[classattr]
        const MONTH_MAR: i16 = constants::TOKEN_MONTH_MAR;
        #[classattr]
        const MONTH_APR: i16 = constants::TOKEN_MONTH_APR;
        #[classattr]
        const MONTH_MAY: i16 = constants::TOKEN_MONTH_MAY;
        #[classattr]
        const MONTH_JUN: i16 = constants::TOKEN_MONTH_JUN;
        #[classattr]
        const MONTH_JUL: i16 = constants::TOKEN_MONTH_JUL;
        #[classattr]
        const MONTH_AUG: i16 = constants::TOKEN_MONTH_AUG;
        #[classattr]
        const MONTH_SEP: i16 = constants::TOKEN_MONTH_SEP;
        #[classattr]
        const MONTH_OCT: i16 = constants::TOKEN_MONTH_OCT;
        #[classattr]
        const MONTH_NOV: i16 = constants::TOKEN_MONTH_NOV;
        #[classattr]
        const MONTH_DEC: i16 = constants::TOKEN_MONTH_DEC;

        #[classattr]
        const UNIT_SEC: i16 = constants::TOKEN_UNIT_SEC;
        #[classattr]
        const UNIT_MIN: i16 = constants::TOKEN_UNIT_MIN;
        #[classattr]
        const UNIT_HRS: i16 = constants::TOKEN_UNIT_HRS;

        #[classattr]
        const SHORT_UNIT_SEC: i16 = constants::TOKEN_SHORT_UNIT_SEC;
        #[classattr]
        const SHORT_UNIT_HRS: i16 = constants::TOKEN_SHORT_UNIT_HRS;
        #[classattr]
        const SHORT_UNIT_DAY: i16 = constants::TOKEN_SHORT_UNIT_DAY;
        #[classattr]
        const SHORT_UNIT_WEEK: i16 = constants::TOKEN_SHORT_UNIT_WEEK;
        #[classattr]
        const SHORT_UNIT_MONTH: i16 = constants::TOKEN_SHORT_UNIT_MONTH;
        #[classattr]
        const SHORT_UNIT_YEAR: i16 = constants::TOKEN_SHORT_UNIT_YEAR;

        #[classattr]
        const LONG_UNIT_SEC: i16 = constants::TOKEN_LONG_UNIT_SEC;
        #[classattr]
        const LONG_UNIT_MIN: i16 = constants::TOKEN_LONG_UNIT_MIN;
        #[classattr]
        const LONG_UNIT_HRS: i16 = constants::TOKEN_LONG_UNIT_HRS;
        #[classattr]
        const LONG_UNIT_DAY: i16 = constants::TOKEN_LONG_UNIT_DAY;
        #[classattr]
        const LONG_UNIT_WEEK: i16 = constants::TOKEN_LONG_UNIT_WEEK;
        #[classattr]
        const LONG_UNIT_MONTH: i16 = constants::TOKEN_LONG_UNIT_MONTH;
        #[classattr]
        const LONG_UNIT_YEAR: i16 = constants::TOKEN_LONG_UNIT_YEAR;

        #[classattr]
        const MERIDIEM_AM: i16 = constants::TOKEN_MERIDIEM_AM;
        #[classattr]
        const MERIDIEM_PM: i16 = constants::TOKEN_MERIDIEM_PM;
    }

    #[pyclass(name = "unit")]
    pub(crate) struct Units {}

    #[pymethods]
    impl Units {
        #[classattr]
        const DAY: &'static str = constants::UNIT_DAY;
        #[classattr]
        const DAYS: &'static str = constants::UNIT_DAYS;
        #[classattr]
        const HOUR: &'static str = constants::UNIT_HOUR;
        #[classattr]
        const HOURS: &'static str = constants::UNIT_HOURS;
        #[classattr]
        const MINUTE: &'static str = constants::UNIT_MINUTE;
        #[classattr]
        const MINUTES: &'static str = constants::UNIT_MINUTES;
        #[classattr]
        const SECOND: &'static str = constants::UNIT_SECOND;
        #[classattr]
        const SECONDS: &'static str = constants::UNIT_SECONDS;
        #[classattr]
        const WEEK: &'static str = constants::UNIT_WEEK;
        #[classattr]
        const WEEKS: &'static str = constants::UNIT_WEEKS;
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
        pass_module,
        signature = (source, today=None, weekday_start_mon=true),
        text_signature = "(source: str, today: datetime.date = None, weekday_start_mon: bool = True) -> datetime.date"
    )]
    fn to_date(
        module: &Bound<'_, PyModule>,
        py: Python,
        source: &str,
        today: Option<Bound<PyDate>>,
        weekday_start_mon: bool,
    ) -> PyResult<NaiveDate> {
        let date_value = &python::into_date(py, today)?;
        let config_patterns = read_config(module)?.patterns;
        let config_tokens = read_tokens(module)?;

        py.allow_threads(move || {
            let result = convert_str(&source, date_value, weekday_start_mon, config_patterns, config_tokens);

            match result {
                Some(v) => Ok(v.date_naive()),
                None => Err(PyValueError::new_err(format!("Unable to convert \"{}\" into datetime", source,))),
            }
        })
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
    ///
    #[pyfunction]
    #[pyo3(
        pass_module,
        signature = (source, now=None, weekday_start_mon=true),
        text_signature = "(source: str, now: datetime.datetime = None, weekday_start_mon: bool = True) -> datetime.datetime"
    )]
    fn to_datetime(
        module: &Bound<'_, PyModule>,
        py: Python,
        source: &str,
        now: Option<Bound<PyDateTime>>,
        weekday_start_mon: bool,
    ) -> PyResult<DateTime<FixedOffset>> {
        let date_value = &python::into_datetime(py, now)?;
        let config_patterns = read_config(module)?.patterns;
        let config_tokens = read_tokens(module)?;

        py.allow_threads(move || {
            let result = convert_str(&source, date_value, weekday_start_mon, config_patterns, config_tokens);

            match result {
                Some(v) => Ok(v),
                None => Err(PyValueError::new_err(format!("Unable to convert \"{}\" into datetime", source,))),
            }
        })
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
        pass_module,
        signature = (seconds, units=None, max="w", min="s"),
        text_signature = "(seconds: float, units: str = None, max: str = 'w', min: str = 's') -> str"
    )]
    fn to_duration(
        module: &Bound<'_, PyModule>,
        py: Python,
        seconds: f64,
        units: Option<&str>,
        max: &str,
        min: &str,
    ) -> PyResult<String> {
        let mut unit_map = units_locale(units.unwrap_or(""));

        match units {
            Some("short") => unit_map.extend(read_config(module)?.units_short),
            Some("long") => unit_map.extend(read_config(module)?.units_long),
            _ => unit_map.extend(read_config(module)?.units),
        }

        py.allow_threads(move || {
            let unit_locale = UnitLocale::from_map(unit_map);
            Ok(convert_duration(seconds, &unit_locale, max, min))
        })
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
        pass_module,
        signature = (source,),
        text_signature = "(source: str) -> float"
    )]
    fn to_seconds(module: &Bound<'_, PyModule>, py: Python, source: &str) -> PyResult<f64> {
        let config_patterns = read_config(module)?.patterns;
        let config_tokens = read_tokens(module)?;

        py.allow_threads(move || {
            let result = convert_seconds(&source, config_patterns, config_tokens);

            match result {
                Ok(v) => Ok(v),
                Err(e) => Err(PyValueError::new_err(e)),
            }
        })
    }

    #[pymodule_init]
    fn init(module: &Bound<'_, PyModule>) -> PyResult<()> {
        module.add(
            ATTR_CONFIG,
            Config {
                patterns: HashMap::new(),
                tokens: HashMap::new(),
                units: units_locale(""),
                units_long: units_locale("long"),
                units_short: units_locale("short"),
            },
        )?;

        Ok(())
    }

    /// Read config registered to Python module
    fn read_config(module: &Bound<'_, PyModule>) -> Result<Config, PyErr> {
        let config = &module.as_borrowed().getattr(ATTR_CONFIG)?.downcast_into::<Config>()?.borrow();

        Ok(Config {
            patterns: config.patterns.clone(),
            tokens: config.tokens.clone(),
            units: config.units.clone(),
            units_long: config.units_long.clone(),
            units_short: config.units_short.clone(),
        })
    }

    /// Read custom tokens registered to Python module, and return
    /// them as tokens the tokenization (currently) accepts
    fn read_tokens(module: &Bound<'_, PyModule>) -> Result<HashMap<String, Token>, PyErr> {
        let config = read_config(module)?;
        let mut result = HashMap::new();

        for (keyword, token_gid) in config.tokens.to_owned() {
            if let Some(token) = gid_into_token(token_gid) {
                result.insert(keyword, token);
            }
        }

        Ok(result)
    }
}

fn units_locale(name: &str) -> HashMap<String, String> {
    match name {
        "long" => HashMap::from([
            (String::from(constants::UNIT_SECOND), String::from("second")),
            (String::from(constants::UNIT_SECONDS), String::from("seconds")),
            (String::from(constants::UNIT_MINUTE), String::from("minute")),
            (String::from(constants::UNIT_MINUTES), String::from("minutes")),
            (String::from(constants::UNIT_HOUR), String::from("hour")),
            (String::from(constants::UNIT_HOURS), String::from("hours")),
            (String::from(constants::UNIT_DAY), String::from("day")),
            (String::from(constants::UNIT_DAYS), String::from("days")),
            (String::from(constants::UNIT_WEEK), String::from("week")),
            (String::from(constants::UNIT_WEEKS), String::from("weeks")),
        ]),
        "short" => HashMap::from([
            (String::from(constants::UNIT_SECOND), String::from("s")),
            (String::from(constants::UNIT_SECONDS), String::from("s")),
            (String::from(constants::UNIT_MINUTE), String::from("min")),
            (String::from(constants::UNIT_MINUTES), String::from("min")),
            (String::from(constants::UNIT_HOUR), String::from("h")),
            (String::from(constants::UNIT_HOURS), String::from("h")),
            (String::from(constants::UNIT_DAY), String::from("d")),
            (String::from(constants::UNIT_DAYS), String::from("d")),
            (String::from(constants::UNIT_WEEK), String::from("w")),
            (String::from(constants::UNIT_WEEKS), String::from("w")),
        ]),
        _ => HashMap::from([
            (String::from(constants::UNIT_SECOND), String::from("sec")),
            (String::from(constants::UNIT_SECONDS), String::from("sec")),
            (String::from(constants::UNIT_MINUTE), String::from("min")),
            (String::from(constants::UNIT_MINUTES), String::from("min")),
            (String::from(constants::UNIT_HOUR), String::from("hr")),
            (String::from(constants::UNIT_HOURS), String::from("hrs")),
            (String::from(constants::UNIT_DAY), String::from("d")),
            (String::from(constants::UNIT_DAYS), String::from("d")),
            (String::from(constants::UNIT_WEEK), String::from("w")),
            (String::from(constants::UNIT_WEEKS), String::from("w")),
        ]),
    }
}
/// Tokenize source string and then convert it into a datetime value
fn convert_str(
    source: &str,
    current_time: &DateTime<FixedOffset>,
    first_weekday_mon: bool,
    custom_patterns: HashMap<String, String>,
    custom_tokens: HashMap<String, Token>,
) -> Option<DateTime<FixedOffset>> {
    let (pattern, tokens) = token::tokenize(&source, custom_tokens);
    fuzzy::convert(&pattern, tokens, &current_time, first_weekday_mon, custom_patterns)
}

/// Convert number of seconds into a time duration string
fn convert_duration(seconds: f64, units: &UnitLocale, max: &str, min: &str) -> String {
    fuzzy::to_duration(seconds, &units, max, min)
}

/// Tokenize source string and then convert it seconds, reflecting exact duration
fn convert_seconds(
    source: &str,
    custom_patterns: HashMap<String, String>,
    custom_tokens: HashMap<String, Token>,
) -> Result<f64, String> {
    let (pattern, tokens) = token::tokenize(&source, custom_tokens);

    if !token::is_time_duration(&pattern) {
        return Err(format!("Unable to convert \"{}\" into seconds", source));
    }

    for token in &tokens {
        if token.token.is_unit() && token.value.eq(&7) {
            return Err(String::from("Converting years into seconds is not supported"));
        }

        if token.token.is_unit() && token.value.eq(&6) {
            return Err(String::from("Converting months into seconds is not supported"));
        }
    }

    let current_time = Utc::now().fixed_offset();

    if let Some(from_time) = fuzzy::convert(&pattern, tokens, &current_time, true, custom_patterns) {
        let duration: Duration = from_time - current_time;
        return Ok((duration.num_milliseconds() / 1_000) as f64);
    }

    Err(format!("Unable to convert \"{}\" into seconds", source))
}

/// Turn global identifier into corresponding tokenization token
fn gid_into_token(gid: u32) -> Option<Token> {
    let gid = gid as i64;

    if gid.ge(&101) && gid.le(&107) {
        return Some(Token::new(TokenType::Weekday, gid - 100));
    }

    if gid.ge(&201) && gid.le(&212) {
        return Some(Token::new(TokenType::Month, gid - 200));
    }

    if gid.ge(&301) && gid.le(&303) {
        return Some(Token::new(TokenType::Unit, gid - 300));
    }

    if gid.ge(&401) && gid.le(&407) && !gid.eq(&402) {
        return Some(Token::new(TokenType::ShortUnit, gid - 400));
    }

    if gid.ge(&501) && gid.le(&507) {
        return Some(Token::new(TokenType::LongUnit, gid - 500));
    }

    if gid.ge(&601) && gid.le(&602) {
        return Some(Token::new(TokenType::Meridiem, gid - 600));
    }

    None
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
            ("@1705072948.9", "2024-01-12 15:22:28.900 +00:00"),
            ("@1705072948.09", "2024-01-12 15:22:28.090 +00:00"),
            ("@1705072948.090", "2024-01-12 15:22:28.090 +00:00"),
            ("@1705072948.099", "2024-01-12 15:22:28.099 +00:00"),
            ("@1705072948.009", "2024-01-12 15:22:28.009 +00:00"),
            ("@1705072948.544", "2024-01-12 15:22:28.544 +00:00"),
            ("00900101", "0090-01-01 00:00:00 +00:00"),
            ("20230101", "2023-01-01 00:00:00 +00:00"),
            ("20241210", "2024-12-10 00:00:00 +00:00"),
            ("2023-01-01", "2023-01-01 00:00:00 +00:00"),
            ("Feb-01-2023", "2023-02-01 00:00:00 +00:00"),
            ("01-Feb-2023", "2023-02-01 00:00:00 +00:00"),
            ("2023-Feb-01", "2023-02-01 00:00:00 +00:00"),
            ("07.02.2023", "2023-02-07 00:00:00 +00:00"),
            ("7.2.2023", "2023-02-07 00:00:00 +00:00"),
            ("2/7/2023", "2023-02-07 00:00:00 +00:00"),
            ("Dec 7 2023", "2023-12-07 00:00:00 +00:00"),
            ("Dec 7th 2023", "2023-12-07 00:00:00 +00:00"),
            ("Dec. 7th 2023", "2023-12-07 00:00:00 +00:00"),
            ("Dec 7th, 2023", "2023-12-07 00:00:00 +00:00"),
            ("7th of December 2023", "2023-12-07 00:00:00 +00:00"),
            ("7th of Dec, 2023", "2023-12-07 00:00:00 +00:00"),
            ("December 7th 2023", "2023-12-07 00:00:00 +00:00"),
            ("7 Dec 2023", "2023-12-07 00:00:00 +00:00"),
            ("7 Dec. 2023", "2023-12-07 00:00:00 +00:00"),
            ("7 December 2023", "2023-12-07 00:00:00 +00:00"),
            ("7. Dec 2023", "2023-12-07 00:00:00 +00:00"),
            ("7. December 2023", "2023-12-07 00:00:00 +00:00"),
            ("2023-12-07 15:02", "2023-12-07 15:02:00 +00:00"),
            ("2023-12-07 15:02:01", "2023-12-07 15:02:01 +00:00"),
            ("2023-12-07 15:02:01", "2023-12-07 15:02:01 +00:00"),
            ("2023-12-07 15:02:01.000", "2023-12-07 15:02:01 +00:00"),
            ("2023-12-07 15:02:01.001", "2023-12-07 15:02:01.001 +00:00"),
            ("2023-12-07 15:02:01.010", "2023-12-07 15:02:01.010 +00:00"),
            ("2023-12-07 15:02:01.04", "2023-12-07 15:02:01.040 +00:00"),
            ("2023-12-07T15:02:01", "2023-12-07 15:02:01 +00:00"),
            ("2023-12-07T15:02:01.04", "2023-12-07 15:02:01.040 +00:00"),
            ("Wed, July 23 2008", "2008-07-23 00:00:00 +00:00"),
            ("Wed, 23 July 2008", "2008-07-23 00:00:00 +00:00"),
            ("Wed, 23rd July 2008", "2008-07-23 00:00:00 +00:00"),
            ("Wed, 23rd of July 2008", "2008-07-23 00:00:00 +00:00"),
            ("Wed, July 23rd 2008", "2008-07-23 00:00:00 +00:00"),
            ("Thu Dec 07 02:00:00 2023", "2023-12-07 02:00:00 +00:00"),
        ];

        let current_time = Utc::now().fixed_offset();

        for (from_string, expect_time) in expect {
            let result_time = convert_str(from_string, &current_time, true, HashMap::new(), HashMap::new());
            assert_eq!(result_time.unwrap().to_string(), expect_time.to_string());
        }
    }

    #[test]
    fn test_fixed_day_month() {
        let expect: Vec<(&str, &str, &str)> = vec![
            ("Sat, Dec 7", "2024-01-12T15:22:28+02:00", "2024-12-07 00:00:00 +02:00"),
            ("Sat, Dec 7th", "2024-01-12T15:22:28+02:00", "2024-12-07 00:00:00 +02:00"),
            ("Sat, 7th of Dec", "2024-01-12T15:22:28+02:00", "2024-12-07 00:00:00 +02:00"),
            ("Sat, 7 Dec", "2024-01-12T15:22:28+02:00", "2024-12-07 00:00:00 +02:00"),
            ("Dec 7", "2024-01-12T15:22:28+02:00", "2024-12-07 00:00:00 +02:00"),
            ("December 7th", "2024-01-12T15:22:28+02:00", "2024-12-07 00:00:00 +02:00"),
            ("7 Dec", "2024-01-12T15:22:28+02:00", "2024-12-07 00:00:00 +02:00"),
            ("7th of Dec", "2024-01-12T15:22:28+02:00", "2024-12-07 00:00:00 +02:00"),
            ("7th of December", "2024-01-12T15:22:28+02:00", "2024-12-07 00:00:00 +02:00"),
        ];

        for (from_string, current_time, expect_time) in expect {
            let current_time = DateTime::parse_from_rfc3339(current_time).unwrap();
            let result_time = convert_str(from_string, &current_time, true, HashMap::new(), HashMap::new());
            assert_eq!(result_time.unwrap().to_string(), expect_time.to_string());
        }
    }

    #[test]
    fn test_fixed_time() {
        let current_time = "2024-01-12T15:22:28+02:00";
        let current_time = DateTime::parse_from_rfc3339(current_time).unwrap();

        let expect: Vec<(&str, &str)> = vec![
            ("12am", "2024-01-12 00:00:00 +02:00"),
            ("12a.m.", "2024-01-12 00:00:00 +02:00"),
            ("12 am", "2024-01-12 00:00:00 +02:00"),
            ("12 a.m.", "2024-01-12 00:00:00 +02:00"),
            ("12:01 am", "2024-01-12 00:01:00 +02:00"),
            ("12pm", "2024-01-12 12:00:00 +02:00"),
            ("12 pm", "2024-01-12 12:00:00 +02:00"),
            ("12:01 pm", "2024-01-12 12:01:00 +02:00"),
            ("12:01 p.m.", "2024-01-12 12:01:00 +02:00"),
            ("1pm", "2024-01-12 13:00:00 +02:00"),
            ("1p.m.", "2024-01-12 13:00:00 +02:00"),
            ("1 pm", "2024-01-12 13:00:00 +02:00"),
            ("1 p.m.", "2024-01-12 13:00:00 +02:00"),
            ("8pm", "2024-01-12 20:00:00 +02:00"),
            ("8 pm", "2024-01-12 20:00:00 +02:00"),
            ("8:01 pm", "2024-01-12 20:01:00 +02:00"),
            ("00:00", "2024-01-12 00:00:00 +02:00"),
            ("00:00:00", "2024-01-12 00:00:00 +02:00"),
            ("23:59:59", "2024-01-12 23:59:59 +02:00"),
            ("3:00", "2024-01-12 03:00:00 +02:00"),
            ("3:00:01", "2024-01-12 03:00:01 +02:00"),
            ("3:00:01.01", "2024-01-12 03:00:01.010 +02:00"),
        ];

        for (from_string, expect_time) in expect {
            let try_string = from_string;
            let result_time = convert_str(try_string, &current_time, true, HashMap::new(), HashMap::new());
            assert_eq!(result_time.unwrap().to_string(), expect_time.to_string());

            let try_string = format!("at {}", from_string);
            let result_time = convert_str(&try_string, &current_time, true, HashMap::new(), HashMap::new());
            assert_eq!(result_time.unwrap().to_string(), expect_time.to_string());

            let try_string = format!("@ {}", from_string);
            let result_time = convert_str(&try_string, &current_time, true, HashMap::new(), HashMap::new());
            assert_eq!(result_time.unwrap().to_string(), expect_time.to_string());
        }
    }

    #[test]
    fn test_keywords() {
        assert_convert_from_mon(vec![
            ("now", "2024-01-12T15:22:28+02:00", "2024-01-12 15:22:28 +02:00"),
            ("midnight", "2024-01-12T15:22:28+02:00", "2024-01-12 00:00:00 +02:00"),
            ("yesterday", "2024-01-12T15:22:28+02:00", "2024-01-11 00:00:00 +02:00"),
            ("tomorrow", "2024-01-12T15:22:28+02:00", "2024-01-13 00:00:00 +02:00"),
        ]);
    }

    #[test]
    fn test_fixed_week_mon() {
        assert_convert_from_mon(vec![
            // ISO8601
            ("2023W01", "2024-05-12T15:22:28+02:00", "2023-01-02 00:00:00 +02:00"),
            ("2023W13", "2024-05-12T15:22:28+02:00", "2023-03-27 00:00:00 +02:00"),
            ("2023-W13", "2024-05-12T15:22:28+02:00", "2023-03-27 00:00:00 +02:00"),
            ("2023-W52", "2024-05-12T15:22:28+02:00", "2023-12-25 00:00:00 +02:00"),
            ("2020-W53", "2024-05-12T15:22:28+02:00", "2020-12-28 00:00:00 +02:00"),
            // Textual
            ("Week 53", "2020-05-12T15:22:28+02:00", "2020-12-28 00:00:00 +02:00"),
            ("Week 1, 2023", "2024-05-12T15:22:28+02:00", "2023-01-02 00:00:00 +02:00"),
            ("Week 13, 2023", "2024-05-12T15:22:28+02:00", "2023-03-27 00:00:00 +02:00"),
        ]);
    }

    #[test]
    fn test_fixed_week_sun() {
        assert_convert_from_sun(vec![
            // ISO8601
            ("2023W01", "2024-05-12T15:22:28+02:00", "2023-01-01 00:00:00 +02:00"),
            ("2023W13", "2024-05-12T15:22:28+02:00", "2023-03-26 00:00:00 +02:00"),
            ("2023-W13", "2024-05-12T15:22:28+02:00", "2023-03-26 00:00:00 +02:00"),
            ("2023-W52", "2024-05-12T15:22:28+02:00", "2023-12-24 00:00:00 +02:00"),
            ("2020-W53", "2024-05-12T15:22:28+02:00", "2020-12-27 00:00:00 +02:00"),
            // Textual
            ("Week 53", "2020-05-12T15:22:28+02:00", "2020-12-27 00:00:00 +02:00"),
            ("Week 1, 2023", "2024-05-12T15:22:28+02:00", "2023-01-01 00:00:00 +02:00"),
            ("Week 13, 2023", "2024-05-12T15:22:28+02:00", "2023-03-26 00:00:00 +02:00"),
        ]);
    }

    #[test]
    fn test_month() {
        assert_convert_from_mon(vec![
            ("Jan", "2024-05-12T15:22:28+02:00", "2024-01-12 00:00:00 +02:00"),
            ("February", "2024-12-30T15:22:28+02:00", "2024-02-29 00:00:00 +02:00"),
        ]);
    }

    #[test]
    fn test_month_year() {
        assert_convert_from_mon(vec![
            ("Jan 2023", "2024-05-12T15:22:28+02:00", "2023-01-01 00:00:00 +02:00"),
            ("February 2025", "2024-12-30T15:22:28+02:00", "2025-02-01 00:00:00 +02:00"),
        ]);
    }

    #[test]
    fn test_month_ranges() {
        assert_convert_from_mon(vec![
            // First
            ("first day of January", "2024-05-12T15:22:28+02:00", "2024-01-01 00:00:00 +02:00"),
            ("first of month", "2024-02-12T15:22:28+02:00", "2024-02-01 00:00:00 +02:00"),
            ("first of the month", "2024-02-12T15:22:28+02:00", "2024-02-01 00:00:00 +02:00"),
            ("first of this month", "2024-02-12T15:22:28+02:00", "2024-02-01 00:00:00 +02:00"),
            ("first day of this month", "2024-02-12T15:22:28+02:00", "2024-02-01 00:00:00 +02:00"),
            ("first day of prev month", "2024-03-12T15:22:28+02:00", "2024-02-01 00:00:00 +02:00"),
            ("first day of last month", "2024-03-12T15:22:28+02:00", "2024-02-01 00:00:00 +02:00"),
            ("first day of next month", "2024-02-12T15:22:28+02:00", "2024-03-01 00:00:00 +02:00"),
            // Last
            ("last day of February", "2024-05-12T15:22:28+02:00", "2024-02-29 00:00:00 +02:00"),
            ("last of month", "2024-02-12T15:22:28+02:00", "2024-02-29 00:00:00 +02:00"),
            ("last of the month", "2024-02-12T15:22:28+02:00", "2024-02-29 00:00:00 +02:00"),
            ("last of this month", "2024-02-12T15:22:28+02:00", "2024-02-29 00:00:00 +02:00"),
            ("last day of this month", "2024-02-12T15:22:28+02:00", "2024-02-29 00:00:00 +02:00"),
            ("last day of prev month", "2024-03-12T15:22:28+02:00", "2024-02-29 00:00:00 +02:00"),
            ("last day of last month", "2024-03-12T15:22:28+02:00", "2024-02-29 00:00:00 +02:00"),
            ("last day of next month", "2023-12-12T15:22:28+02:00", "2024-01-31 00:00:00 +02:00"),
        ]);
    }

    #[test]
    fn test_month_year_ranges() {
        assert_convert_from_mon(vec![
            ("first day of January 2027", "2024-05-12T15:22:28+02:00", "2027-01-01 00:00:00 +02:00"),
            ("last day of February 2025", "2026-05-12T15:22:28+02:00", "2025-02-28 00:00:00 +02:00"),
        ]);
    }

    #[test]
    fn test_wday_ranges() {
        assert_convert_from_mon(vec![
            ("first mon of Feb", "2024-05-12T15:22:28+02:00", "2024-02-05 00:00:00 +02:00"),
            ("first tue of 2025", "2024-05-12T15:22:28+02:00", "2025-01-07 00:00:00 +02:00"),
            ("first wed of Jan 2025", "2026-05-12T15:22:28+02:00", "2025-01-01 00:00:00 +02:00"),
            ("first tue of Jan 2025", "2026-05-12T15:22:28+02:00", "2025-01-07 00:00:00 +02:00"),
            ("last mon of Feb", "2024-05-12T15:22:28+02:00", "2024-02-26 00:00:00 +02:00"),
            ("last fri of 2025", "2024-05-12T15:22:28+02:00", "2025-12-26 00:00:00 +02:00"),
            ("last fri of Jan 2025", "2026-05-12T15:22:28+02:00", "2025-01-31 00:00:00 +02:00"),
            ("last sat of Jan 2025", "2026-05-12T15:22:28+02:00", "2025-01-25 00:00:00 +02:00"),
        ]);
    }

    #[test]
    fn test_year_ranges() {
        assert_convert_from_mon(vec![
            // First
            ("first day of this year", "2024-02-12T15:22:28+02:00", "2024-01-01 00:00:00 +02:00"),
            ("first day of prev year", "2024-03-12T15:22:28+02:00", "2023-01-01 00:00:00 +02:00"),
            ("first day of last year", "2024-03-12T15:22:28+02:00", "2023-01-01 00:00:00 +02:00"),
            ("first day of next year", "2024-02-12T15:22:28+02:00", "2025-01-01 00:00:00 +02:00"),
            // Last
            ("last day of this year", "2024-02-12T15:22:28+02:00", "2024-12-31 00:00:00 +02:00"),
            ("last day of prev year", "2024-03-12T15:22:28+02:00", "2023-12-31 00:00:00 +02:00"),
            ("last day of last year", "2024-03-12T15:22:28+02:00", "2023-12-31 00:00:00 +02:00"),
            ("last day of next year", "2024-02-12T15:22:28+02:00", "2025-12-31 00:00:00 +02:00"),
            // Specific year
            ("first day of 2025", "2024-02-12T15:22:28+02:00", "2025-01-01 00:00:00 +02:00"),
            ("last day of 2025", "2024-02-12T15:22:28+02:00", "2025-12-31 00:00:00 +02:00"),
        ]);
    }

    #[test]
    fn test_offset_seconds() {
        assert_convert_from_mon(vec![
            ("this second", "2024-01-12T15:22:28+02:00", "2024-01-12 15:22:28 +02:00"),
            ("past second", "2024-01-12T15:22:28+02:00", "2024-01-12 15:22:27 +02:00"),
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
            ("past 1 seconds", "2024-01-25T15:22:28+02:00", "2024-01-25 15:22:27 +02:00"),
            ("prev 2 seconds", "2024-01-25T15:22:28+02:00", "2024-01-25 15:22:26 +02:00"),
            ("last 1 seconds", "2024-01-25T15:22:28+02:00", "2024-01-25 15:22:27 +02:00"),
        ]);
    }

    #[test]
    fn test_offset_minutes() {
        assert_convert_from_mon(vec![
            ("this minute", "2024-01-12T15:22:28+02:00", "2024-01-12 15:22:28 +02:00"),
            ("past minute", "2024-01-12T15:22:28+02:00", "2024-01-12 15:21:28 +02:00"),
            ("prev minute", "2024-01-12T15:22:28+02:00", "2024-01-12 15:21:28 +02:00"),
            ("last minute", "2024-01-12T15:22:28+02:00", "2024-01-12 15:21:28 +02:00"),
            ("next minute", "2024-01-12T15:22:28+02:00", "2024-01-12 15:23:28 +02:00"),
            ("-1min", "2024-01-12T15:22:28+02:00", "2024-01-12 15:21:28 +02:00"),
            ("-5 minutes", "2024-01-12 15:22:28+02:00", "2024-01-12 15:17:28 +02:00"),
            ("+60min", "2024-01-12T15:22:28+02:00", "2024-01-12 16:22:28 +02:00"),
            ("+60 minutes", "2024-01-12T15:22:28+02:00", "2024-01-12 16:22:28 +02:00"),
            ("1 min ago", "2024-01-12T15:22:28+02:00", "2024-01-12 15:21:28 +02:00"),
            ("5 minutes ago", "2024-01-12 15:22:28+02:00", "2024-01-12 15:17:28 +02:00"),
            ("past 5 minutes", "2024-01-12 15:22:28+02:00", "2024-01-12 15:17:28 +02:00"),
            ("prev 5 minutes", "2024-01-12 15:22:28+02:00", "2024-01-12 15:17:28 +02:00"),
            ("last 5 minutes", "2024-01-12 15:22:28+02:00", "2024-01-12 15:17:28 +02:00"),
        ]);
    }

    #[test]
    fn test_offset_hours() {
        assert_convert_from_mon(vec![
            ("this hour", "2024-01-12T15:22:28+02:00", "2024-01-12 15:22:28 +02:00"),
            ("past hour", "2024-01-12T15:22:28+02:00", "2024-01-12 14:22:28 +02:00"),
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
            ("past 1 hour", "2024-01-12T15:22:28+02:00", "2024-01-12 14:22:28 +02:00"),
            ("prev 1 hour", "2024-01-12T15:22:28+02:00", "2024-01-12 14:22:28 +02:00"),
            ("last 1 hour", "2024-01-12T15:22:28+02:00", "2024-01-12 14:22:28 +02:00"),
        ]);
    }

    #[test]
    fn test_offset_days() {
        assert_convert_from_mon(vec![
            ("this day", "2024-01-12T15:22:28+02:00", "2024-01-12 15:22:28 +02:00"),
            ("past day", "2024-01-12T15:22:28+02:00", "2024-01-11 15:22:28 +02:00"),
            ("prev day", "2024-01-12T15:22:28+02:00", "2024-01-11 15:22:28 +02:00"),
            ("last day", "2024-01-12T15:22:28+02:00", "2024-01-11 15:22:28 +02:00"),
            ("next day", "2024-01-12T15:22:28+02:00", "2024-01-13 15:22:28 +02:00"),
            ("-1d", "2024-01-12T15:22:28+02:00", "2024-01-11 15:22:28 +02:00"),
            ("-1 day", "2024-01-12T15:22:28+02:00", "2024-01-11 15:22:28 +02:00"),
            ("+1d", "2024-01-12T15:22:28+02:00", "2024-01-13 15:22:28 +02:00"),
            ("+30 days", "2024-01-12T15:22:28+02:00", "2024-02-11 15:22:28 +02:00"),
            ("2 days ago", "2024-01-12T15:22:28+02:00", "2024-01-10 15:22:28 +02:00"),
            ("past 2 days", "2024-01-12T15:22:28+02:00", "2024-01-10 15:22:28 +02:00"),
            ("prev 2 days", "2024-01-12T15:22:28+02:00", "2024-01-10 15:22:28 +02:00"),
            ("last 2 days", "2024-01-12T15:22:28+02:00", "2024-01-10 15:22:28 +02:00"),
        ]);
    }

    #[test]
    fn test_offset_weekdays() {
        assert_convert_from_mon(vec![
            ("Monday", "2024-05-12T15:22:28+02:00", "2024-05-13 00:00:00 +02:00"),
            ("this MONDAY", "2024-05-12T15:22:28+02:00", "2024-05-06 00:00:00 +02:00"),
            ("this Sunday", "2024-01-19T15:22:28+02:00", "2024-01-21 00:00:00 +02:00"),
            ("prev Sunday", "2024-01-19T15:22:28+02:00", "2024-01-14 00:00:00 +02:00"),
            ("last Mon", "2024-01-19T15:22:28+02:00", "2024-01-15 00:00:00 +02:00"),
            ("next Mon", "2024-01-19T15:22:28+02:00", "2024-01-22 00:00:00 +02:00"),
            ("next Sunday", "2024-01-19T15:22:28+02:00", "2024-01-21 00:00:00 +02:00"),
            // Current weekday is the same as new weekday
            ("this Saturday", "2024-01-20T15:22:28+02:00", "2024-01-20 00:00:00 +02:00"),
            ("prev Saturday", "2024-01-20T15:22:28+02:00", "2024-01-13 00:00:00 +02:00"),
            ("next Saturday", "2024-01-20T15:22:28+02:00", "2024-01-27 00:00:00 +02:00"),
        ]);
    }

    #[test]
    fn test_offset_weeks_exact() {
        assert_convert_from_mon(vec![
            ("-1w", "2024-01-25T15:22:28+02:00", "2024-01-18 15:22:28 +02:00"),
            ("-2 weeks", "2024-01-25T15:22:28+02:00", "2024-01-11 15:22:28 +02:00"),
            ("+1w", "2024-01-14T14:22:28+02:00", "2024-01-21 14:22:28 +02:00"),
            ("+2 weeks", "2024-01-08T15:22:28+02:00", "2024-01-22 15:22:28 +02:00"),
            ("1 week ago", "2024-01-25T15:22:28+02:00", "2024-01-18 15:22:28 +02:00"),
            ("past week", "2024-01-25T15:22:28+02:00", "2024-01-18 15:22:28 +02:00"),
            ("past 2 weeks", "2024-01-25T15:22:28+02:00", "2024-01-11 15:22:28 +02:00"),
        ]);
    }

    #[test]
    fn test_offset_weeks_monday() {
        let expect: Vec<(&str, &str, &str)> = vec![
            ("this week", "2024-01-25T15:22:28+02:00", "2024-01-22 15:22:28 +02:00"),
            ("prev week", "2024-01-25T15:22:28+02:00", "2024-01-15 15:22:28 +02:00"),
            ("last week", "2024-01-25T15:22:28+02:00", "2024-01-15 15:22:28 +02:00"),
            ("next week", "2024-01-13 15:22:28+02:00", "2024-01-15 15:22:28 +02:00"),
            ("prev 2 weeks", "2024-01-25T15:22:28+02:00", "2024-01-08 15:22:28 +02:00"),
            ("last 2 weeks", "2024-01-25T15:22:28+02:00", "2024-01-08 15:22:28 +02:00"),
        ];

        for (from_string, current_time, expect_time) in expect {
            let current_time = DateTime::parse_from_rfc3339(current_time).unwrap();
            let result_time = convert_str(from_string, &current_time, true, HashMap::new(), HashMap::new());
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
            ("prev 2 weeks", "2024-01-25T15:22:28+02:00", "2024-01-07 15:22:28 +02:00"),
            ("last 2 weeks", "2024-01-25T15:22:28+02:00", "2024-01-07 15:22:28 +02:00"),
        ];

        for (from_string, current_time, expect_time) in expect {
            let current_time = DateTime::parse_from_rfc3339(current_time).unwrap();
            let result_time = convert_str(from_string, &current_time, false, HashMap::new(), HashMap::new());
            assert_eq!(result_time.unwrap().to_string(), expect_time.to_string())
        }
    }

    #[test]
    fn test_offset_month() {
        assert_convert_from_mon(vec![
            ("this April", "2024-01-19T15:22:28+02:00", "2024-04-19 00:00:00 +02:00"),
            ("prev April", "2024-01-19T15:22:28+02:00", "2023-04-19 00:00:00 +02:00"),
            ("last April", "2024-01-19T15:22:28+02:00", "2023-04-19 00:00:00 +02:00"),
            ("next April", "2024-01-19T15:22:28+02:00", "2024-04-19 00:00:00 +02:00"),
            ("next January", "2024-01-19T15:22:28+02:00", "2025-01-19 00:00:00 +02:00"),
            // When current month is the same as new month
            ("this April", "2024-04-15T15:22:28+02:00", "2024-04-15 00:00:00 +02:00"),
            ("prev April", "2024-04-15T15:22:28+02:00", "2023-04-15 00:00:00 +02:00"),
            ("next April", "2024-04-15T15:22:28+02:00", "2025-04-15 00:00:00 +02:00"),
        ]);
    }

    #[test]
    fn test_offset_months() {
        assert_convert_from_mon(vec![
            ("this month", "2024-03-12T15:22:28+02:00", "2024-03-12 15:22:28 +02:00"),
            ("past month", "2024-03-12T15:22:28+02:00", "2024-02-12 15:22:28 +02:00"),
            ("prev month", "2024-03-12T15:22:28+02:00", "2024-02-12 15:22:28 +02:00"),
            ("last month", "2024-03-12T15:22:28+02:00", "2024-02-12 15:22:28 +02:00"),
            ("next month", "2024-12-12T15:22:28+02:00", "2025-01-12 15:22:28 +02:00"),
            ("-1m", "2024-03-12T15:22:28+02:00", "2024-02-12 15:22:28 +02:00"),
            ("-1 month", "2024-03-12T15:22:28+02:00", "2024-02-12 15:22:28 +02:00"),
            ("+1m", "2024-03-12T15:22:28+02:00", "2024-04-12 15:22:28 +02:00"),
            ("+13 months", "2023-12-12T15:22:28+02:00", "2025-01-12 15:22:28 +02:00"),
            ("1 month ago", "2024-03-12T15:22:28+02:00", "2024-02-12 15:22:28 +02:00"),
            ("past 1 months", "2024-03-12T15:22:28+02:00", "2024-02-12 15:22:28 +02:00"),
            ("prev 1 months", "2024-03-12T15:22:28+02:00", "2024-02-12 15:22:28 +02:00"),
            ("last 1 months", "2024-03-12T15:22:28+02:00", "2024-02-12 15:22:28 +02:00"),
            // Different number of days in each month
            ("-1m", "2022-05-31T15:22:28+02:00", "2022-04-30 15:22:28 +02:00"),
        ]);
    }

    #[test]
    fn test_offset_years() {
        assert_convert_from_mon(vec![
            ("this year", "2024-01-12T15:22:28+02:00", "2024-01-12 15:22:28 +02:00"),
            ("past year", "2024-01-12T15:22:28+02:00", "2023-01-12 15:22:28 +02:00"),
            ("prev year", "2024-01-12T15:22:28+02:00", "2023-01-12 15:22:28 +02:00"),
            ("last year", "2024-01-12T15:22:28+02:00", "2023-01-12 15:22:28 +02:00"),
            ("next year", "2024-01-12T15:22:28+02:00", "2025-01-12 15:22:28 +02:00"),
            ("-1y", "2024-01-12T15:22:28+02:00", "2023-01-12 15:22:28 +02:00"),
            ("-1 year", "2024-01-12T15:22:28+02:00", "2023-01-12 15:22:28 +02:00"),
            ("+1y", "2024-01-12T15:22:28+02:00", "2025-01-12 15:22:28 +02:00"),
            ("+10 years", "2024-01-12T15:22:28+02:00", "2034-01-12 15:22:28 +02:00"),
            ("2 years ago", "2024-01-12T15:22:28+02:00", "2022-01-12 15:22:28 +02:00"),
            ("past 2 years", "2024-01-12T15:22:28+02:00", "2022-01-12 15:22:28 +02:00"),
            ("prev 2 years", "2024-01-12T15:22:28+02:00", "2022-01-12 15:22:28 +02:00"),
            ("last 2 years", "2024-01-12T15:22:28+02:00", "2022-01-12 15:22:28 +02:00"),
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
        assert_convert_from_mon(vec![
            ("@1705072948.544 2pm", "2024-01-12T15:22:28+02:00", "2024-01-12 14:00:00 +00:00"),
            ("2023-12-07 3p.m.", "2024-01-12T15:22:28+02:00", "2023-12-07 15:00:00 +02:00"),
            ("2023-12-07 3 p.m.", "2024-01-12T15:22:28+02:00", "2023-12-07 15:00:00 +02:00"),
            ("2023-12-07 3:00 p.m.", "2024-01-12T15:22:28+02:00", "2023-12-07 15:00:00 +02:00"),
            ("dec 2pm", "2024-01-12T15:22:28+02:00", "2024-12-12 14:00:00 +02:00"),
            ("dec 12.3.2023", "2024-01-12T15:22:28+02:00", "2023-03-12 00:00:00 +02:00"),
            ("dec 2025 2pm", "2024-01-12T15:22:28+02:00", "2025-12-01 14:00:00 +02:00"),
            ("Nov-09-2006 2pm", "2024-01-12T15:22:28+02:00", "2006-11-09 14:00:00 +02:00"),
            ("Nov-09-2006 at 2pm", "2024-01-12T15:22:28+02:00", "2006-11-09 14:00:00 +02:00"),
            ("Nov 2006 2pm", "2024-01-12T15:22:28+02:00", "2006-11-01 14:00:00 +02:00"),
            ("Nov 2006 @ 2pm", "2024-01-12T15:22:28+02:00", "2006-11-01 14:00:00 +02:00"),
            ("20240210 2pm", "2024-01-12T15:22:28+02:00", "2024-02-10 14:00:00 +02:00"),
            ("yesterday 1pm", "2024-01-12T15:22:28+02:00", "2024-01-11 13:00:00 +02:00"),
            ("yesterday 1:00 pm", "2024-01-12T15:22:28+02:00", "2024-01-11 13:00:00 +02:00"),
            ("yesterday midnight", "2024-01-12T15:22:28+02:00", "2024-01-11 00:00:00 +02:00"),
            ("-2d 1h", "2024-05-12T15:22:28+02:00", "2024-05-10 14:22:28 +02:00"),
            ("-2d 1h midnight", "2024-05-12T15:22:28+02:00", "2024-05-10 00:00:00 +02:00"),
            ("first day of Jan last year", "2024-05-12T15:22:28+02:00", "2023-01-01 00:00:00 +02:00"),
            ("last day of Feb last year", "2024-05-12T15:22:28+02:00", "2023-02-28 00:00:00 +02:00"),
            ("2 months 5 days 1 hour ago", "2024-05-12T15:22:28+02:00", "2024-03-07 14:22:28 +02:00"),
            ("Wed, 23 July 2008 12:00:00", "2024-05-12T15:22:28+02:00", "2008-07-23 12:00:00 +02:00"),
        ]);
    }

    #[test]
    fn test_combinations_time_of_day_rules() {
        assert_convert_from_mon(vec![
            // Time of day is processed after date
            ("2:00 pm today", "2024-05-12T15:22:28+02:00", "2024-05-12 14:00:00 +02:00"),
            ("14:00 2024-05-17", "2024-05-12T15:22:28+02:00", "2024-05-17 14:00:00 +02:00"),
            ("14:00:00 2024-05-17", "2024-05-12T15:22:28+02:00", "2024-05-17 14:00:00 +02:00"),
            ("14:00:00 May 17th 2024", "2024-05-12T15:22:28+02:00", "2024-05-17 14:00:00 +02:00"),
            // Time of day is processed after weekday
            ("monday 2pm", "2024-05-12T15:22:28+02:00", "2024-05-13 14:00:00 +02:00"),
            ("2pm monday", "2024-05-12T15:22:28+02:00", "2024-05-13 14:00:00 +02:00"),
            ("2:00 pm monday", "2024-05-12T15:22:28+02:00", "2024-05-13 14:00:00 +02:00"),
            ("14:00 monday", "2024-05-12T15:22:28+02:00", "2024-05-13 14:00:00 +02:00"),
            ("14:00:00 monday", "2024-05-12T15:22:28+02:00", "2024-05-13 14:00:00 +02:00"),
            ("14:00:00.00 monday", "2024-05-12T15:22:28+02:00", "2024-05-13 14:00:00 +02:00"),
            // Time of day is processed after month and day
            ("2015 2pm Feb 1", "2024-05-12T15:22:28+02:00", "2015-02-01 14:00:00 +02:00"),
            ("2015 12:00:00 Feb 1", "2024-05-12T15:22:28+02:00", "2015-02-01 12:00:00 +02:00"),
            // Plus/minus movement is processed after time of day
            ("2pm +10 minutes", "2024-05-12T15:22:28+02:00", "2024-05-12 14:10:00 +02:00"),
            // Midnight enforces reset
            ("2pm midnight", "2024-05-12T15:22:28+02:00", "2024-05-12 00:00:00 +02:00"),
        ]);
    }

    #[test]
    fn test_combinations_weekday_rules() {
        assert_convert_from_mon(vec![
            ("sunday", "2024-05-12T15:22:28+02:00", "2024-05-12 00:00:00 +02:00"),
            ("monday", "2024-05-12T15:22:28+02:00", "2024-05-13 00:00:00 +02:00"),
            // Weekday is processed after long unit
            ("monday next week", "2024-05-12T15:22:28+02:00", "2024-05-13 00:00:00 +02:00"),
            ("tuesday next week", "2024-05-12T15:22:28+02:00", "2024-05-14 00:00:00 +02:00"),
            ("monday this week", "2024-05-12T15:22:28+02:00", "2024-05-06 00:00:00 +02:00"),
            ("tuesday last week", "2024-05-12T15:22:28+02:00", "2024-04-30 00:00:00 +02:00"),
            ("tuesday past week", "2024-05-12T15:22:28+02:00", "2024-05-07 00:00:00 +02:00"),
            ("last week monday", "2024-05-12T15:22:28+02:00", "2024-04-29 00:00:00 +02:00"),
            ("this week thursday", "2024-05-12T15:22:28+02:00", "2024-05-09 00:00:00 +02:00"),
            ("next week monday", "2024-05-12T15:22:28+02:00", "2024-05-13 00:00:00 +02:00"),
            ("next week thursday", "2024-05-12T15:22:28+02:00", "2024-05-16 00:00:00 +02:00"),
        ]);
    }

    #[test]
    fn test_combinations_year_rules() {
        assert_convert_from_mon(vec![
            ("2015", "2024-02-12T15:22:28+02:00", "2015-02-12 15:22:28 +02:00"),
            ("2023", "2024-02-29T15:22:28+02:00", "2023-02-28 15:22:28 +02:00"),
            ("2015, Feb 1", "2024-05-12T15:22:28+02:00", "2015-02-01 00:00:00 +02:00"),
            ("2015 today", "2024-05-12T15:22:28+02:00", "2015-05-12 00:00:00 +02:00"),
            ("2015 next year", "2024-05-12T15:22:28+02:00", "2016-05-12 15:22:28 +02:00"),
            ("2015 next year -1 month", "2024-05-12T15:22:28+02:00", "2016-04-12 15:22:28 +02:00"),
            ("next year -1 month 2015", "2024-05-12T15:22:28+02:00", "2015-04-12 15:22:28 +02:00"),
            ("next year 2015", "2024-05-12T15:22:28+02:00", "2015-05-12 15:22:28 +02:00"),
        ]);

        assert_convert_failure(vec![
            // Year can't be used together with patterns that
            // have their own year defined
            "2015 @1705072948",
            "2015 @1705072948.544",
            "2015 00900101",
            "20230101 2015",
            "2015 2023-01-01",
            "2015 7.2.2023",
            "2015 2/7/2023",
            "2015 2023-12-07 15:02",
            "2015 2023-12-07 15:02:01",
            "2015 2023-12-07 15:02:01.000",
            "2015 2023-12-07T15:02:01",
            "2015 2023-12-07T15:02:01.04",
            "2015 Wed, July 23 2008",
            "2015 Wed, 23 July 2008",
            "2015 Thu Dec 07 02:00:00 2023",
            "2015 2015",
        ]);
    }

    #[test]
    fn test_unsupported() {
        assert_convert_failure(vec![
            "",                          // Not parsed
            " ",                         // Nothing to parse
            "2024-12-01 7",              // Unknown part
            "+1day",                     // Not recognized
            "0000-01-12 15:22",          // Year invalid
            "2024-W0",                   // Week invalid
            "2025-W53",                  // Week invalid
            "1982-04-32",                // Date invalid
            "1982-04-01 15:61",          // Time invalid
            "1995-07-01 12:00:00.10000", // Milliseconds invalid
            "1995-07-01 12:00:00.1000",  // Milliseconds invalid
            "1995-07-01 12:00:00.0001",  // Milliseconds invalid
            "1995-07-01 12:00:00.0010",  // Milliseconds invalid
            "1995-07-01 12:00:00.0230",  // Milliseconds invalid
            "2023-12-07t15:02:01",       // Lowercase T not supported
            "2023w01",                   // Lowercase W not supported
            "Feb 29th 2023",             // Day out of range
            "first of year",             // Not supported
            "first of the year",         // Not supported
            "first day of this week",    // Not supported
            "first minute of Jan",       // Not supported
            "7 of Jan",                  // Missing nth supported
            "Tue, 23 July 2008",         // Wrong weekday with date pattern
            "2008 Tue 23 July",          // Wrong weekday with year pattern
            "Tue, 7 Dec",                // Wrong weekday
            "Fri Dec 07 02:00:00 2023",  // Wrong weekday
            "23:61:00",                  // Invalid time of day
            "tuesday 2023-05-01",        // Invalid use of weekday
            "month 7, 2023",             // Invalid unit for syntax
        ])
    }

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
            let result_value = convert_seconds(from_string, HashMap::new(), HashMap::new());
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
            let result_value = convert_seconds(from_string, HashMap::new(), HashMap::new());
            assert!(result_value.is_err());
        }
    }

    #[test]
    fn test_gid_into_token() {
        for value in 101..=107 {
            assert_eq!(gid_into_token(value).unwrap(), Token::new(TokenType::Weekday, value as i64 - 100));
        }
        assert!(gid_into_token(100).is_none());
        assert!(gid_into_token(108).is_none());

        for value in 201..=212 {
            assert_eq!(gid_into_token(value).unwrap(), Token::new(TokenType::Month, value as i64 - 200));
        }
        assert!(gid_into_token(200).is_none());
        assert!(gid_into_token(213).is_none());

        for value in 301..=303 {
            assert_eq!(gid_into_token(value).unwrap(), Token::new(TokenType::Unit, value as i64 - 300));
        }
        assert!(gid_into_token(300).is_none());
        assert!(gid_into_token(304).is_none());

        for value in 401..=407 {
            if !value.eq(&402) {
                assert_eq!(gid_into_token(value).unwrap(), Token::new(TokenType::ShortUnit, value as i64 - 400));
            }
        }
        assert!(gid_into_token(400).is_none());
        assert!(gid_into_token(408).is_none());

        for value in 501..=507 {
            assert_eq!(gid_into_token(value).unwrap(), Token::new(TokenType::LongUnit, value as i64 - 500));
        }
        assert!(gid_into_token(500).is_none());
        assert!(gid_into_token(508).is_none());

        for value in 601..=602 {
            assert_eq!(gid_into_token(value).unwrap(), Token::new(TokenType::Meridiem, value as i64 - 600));
        }
        assert!(gid_into_token(600).is_none());
        assert!(gid_into_token(603).is_none());
    }

    fn assert_convert_failure(expect: Vec<&str>) {
        let current_time = "2024-01-12T15:22:28+02:00";
        let current_time = DateTime::parse_from_rfc3339(current_time).unwrap();

        for from_string in expect {
            let result_time = convert_str(from_string, &current_time, true, HashMap::new(), HashMap::new());
            assert!(result_time.is_none());
        }
    }

    fn assert_convert_from_mon(expect: Vec<(&str, &str, &str)>) {
        for (from_string, current_time, expect_time) in expect {
            let current_time = DateTime::parse_from_rfc3339(current_time).unwrap();
            let result_time = convert_str(from_string, &current_time, true, HashMap::new(), HashMap::new());
            assert_eq!(result_time.unwrap().to_string(), expect_time.to_string());
        }
    }

    fn assert_convert_from_sun(expect: Vec<(&str, &str, &str)>) {
        for (from_string, current_time, expect_time) in expect {
            let current_time = DateTime::parse_from_rfc3339(current_time).unwrap();
            let result_time = convert_str(from_string, &current_time, false, HashMap::new(), HashMap::new());
            assert_eq!(result_time.unwrap().to_string(), expect_time.to_string());
        }
    }

    fn assert_to_duration(max: &str, min: &str, expect: Vec<(f64, &str, &str)>) {
        for (from_seconds, unit_types, expect_str) in expect {
            let unit_locale = UnitLocale::from_map(units_locale(unit_types));
            let into_duration = convert_duration(from_seconds, &unit_locale, max, min);
            assert_eq!(into_duration, expect_str);

            if max.eq("") && min.eq("") && expect_str.len().gt(&0) {
                let into_seconds = convert_seconds(expect_str, HashMap::new(), HashMap::new());
                assert_eq!(into_seconds.unwrap(), from_seconds);
            }
        }
    }
}
