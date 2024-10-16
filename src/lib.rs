mod convert;
mod fuzzy;
mod token;
mod python;
mod constants;

use crate::token::Token;
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
            fn add_patterns(
                &mut self,
                patterns: HashMap<String, String>) -> PyResult<()> {
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
            fn add_tokens(
                &mut self,
                tokens: HashMap<String, u32>) -> PyResult<()> {
                for (keyword, gid) in tokens {
                    if gid_into_token(gid).is_some() {
                        self.tokens.insert(keyword.to_lowercase(), gid);
                        continue;
                    }

                    return Err(PyValueError::new_err(format!(
                        "Token \"{}\" value {} does not exist", keyword, gid,
                    )));
                }

                Ok(())
            }
        }
    }

    #[pyclass(name = "pattern")]
    pub(crate) struct Patterns {}

    #[pymethods]
    impl Patterns {
        // @formatter:off

        #[classattr] const NOW: &'static str = constants::PATTERN_NOW;
        #[classattr] const TODAY: &'static str = constants::PATTERN_TODAY;
        #[classattr] const MIDNIGHT: &'static str = constants::PATTERN_MIDNIGHT;
        #[classattr] const YESTERDAY: &'static str = constants::PATTERN_YESTERDAY;
        #[classattr] const TOMORROW: &'static str = constants::PATTERN_TOMORROW;

        #[classattr] const THIS_WDAY: &'static str = constants::PATTERN_THIS_WDAY;
        #[classattr] const PREV_WDAY: &'static str = constants::PATTERN_PREV_WDAY;
        #[classattr] const LAST_WDAY: &'static str = constants::PATTERN_LAST_WDAY;
        #[classattr] const NEXT_WDAY: &'static str = constants::PATTERN_NEXT_WDAY;

        #[classattr] const THIS_LONG_UNIT: &'static str = constants::PATTERN_THIS_LONG_UNIT;
        #[classattr] const PREV_LONG_UNIT: &'static str = constants::PATTERN_PREV_LONG_UNIT;
        #[classattr] const LAST_LONG_UNIT: &'static str = constants::PATTERN_LAST_LONG_UNIT;
        #[classattr] const NEXT_LONG_UNIT: &'static str = constants::PATTERN_NEXT_LONG_UNIT;

        #[classattr] const MINUS_UNIT: &'static str = constants::PATTERN_MINUS_UNIT;
        #[classattr] const MINUS_SHORT_UNIT: &'static str = constants::PATTERN_MINUS_SHORT_UNIT;
        #[classattr] const MINUS_LONG_UNIT: &'static str = constants::PATTERN_MINUS_LONG_UNIT;

        #[classattr] const PLUS_UNIT: &'static str = constants::PATTERN_PLUS_UNIT;
        #[classattr] const PLUS_SHORT_UNIT: &'static str = constants::PATTERN_PLUS_SHORT_UNIT;
        #[classattr] const PLUS_LONG_UNIT: &'static str = constants::PATTERN_PLUS_LONG_UNIT;
        #[classattr] const UNIT_AGO: &'static str = constants::PATTERN_UNIT_AGO;
        #[classattr] const LONG_UNIT_AGO: &'static str = constants::PATTERN_LONG_UNIT_AGO;

        #[classattr] const FIRST_LONG_UNIT_OF_MONTH: &'static str = constants::PATTERN_FIRST_LONG_UNIT_OF_MONTH;
        #[classattr] const LAST_LONG_UNIT_OF_MONTH: &'static str = constants::PATTERN_LAST_LONG_UNIT_OF_MONTH;
        #[classattr] const FIRST_LONG_UNIT_OF_THIS_LONG_UNIT: &'static str = constants::PATTERN_FIRST_LONG_UNIT_OF_THIS_LONG_UNIT;
        #[classattr] const LAST_LONG_UNIT_OF_THIS_LONG_UNIT: &'static str = constants::PATTERN_LAST_LONG_UNIT_OF_THIS_LONG_UNIT;
        #[classattr] const FIRST_LONG_UNIT_OF_PREV_LONG_UNIT: &'static str = constants::PATTERN_FIRST_LONG_UNIT_OF_PREV_LONG_UNIT;
        #[classattr] const LAST_LONG_UNIT_OF_PREV_LONG_UNIT: &'static str = constants::PATTERN_LAST_LONG_UNIT_OF_PREV_LONG_UNIT;
        #[classattr] const FIRST_LONG_UNIT_OF_LAST_LONG_UNIT: &'static str = constants::PATTERN_FIRST_LONG_UNIT_OF_LAST_LONG_UNIT;
        #[classattr] const LAST_LONG_UNIT_OF_LAST_LONG_UNIT: &'static str = constants::PATTERN_LAST_LONG_UNIT_OF_LAST_LONG_UNIT;
        #[classattr] const FIRST_LONG_UNIT_OF_NEXT_LONG_UNIT: &'static str = constants::PATTERN_FIRST_LONG_UNIT_OF_NEXT_LONG_UNIT;
        #[classattr] const LAST_LONG_UNIT_OF_NEXT_LONG_UNIT: &'static str = constants::PATTERN_LAST_LONG_UNIT_OF_NEXT_LONG_UNIT;

        #[classattr] const TIMESTAMP: &'static str = constants::PATTERN_TIMESTAMP;
        #[classattr] const TIMESTAMP_FLOAT: &'static str = constants::PATTERN_TIMESTAMP_FLOAT;

        #[classattr] const DATE_YMD: &'static str = constants::PATTERN_DATE_YMD;
        #[classattr] const DATE_DMY: &'static str = constants::PATTERN_DATE_DMY;
        #[classattr] const DATE_MDY: &'static str = constants::PATTERN_DATE_MDY;

        #[classattr] const DATE_MONTH_DAY: &'static str = constants::PATTERN_DATE_MONTH_DAY;
        #[classattr] const DATE_MONTH_DAY_YEAR: &'static str = constants::PATTERN_DATE_MONTH_DAY_YEAR;
        #[classattr] const DATE_MONTH_NTH: &'static str = constants::PATTERN_DATE_MONTH_NTH;
        #[classattr] const DATE_MONTH_NTH_YEAR: &'static str = constants::PATTERN_DATE_MONTH_NTH_YEAR;
        #[classattr] const DATE_DAY_MONTH: &'static str = constants::PATTERN_DATE_DAY_MONTH;
        #[classattr] const DATE_DAY_MONTH_YEAR: &'static str = constants::PATTERN_DATE_DAY_MONTH_YEAR;

        #[classattr] const DATETIME_YMD_HM: &'static str = constants::PATTERN_DATETIME_YMD_HM;
        #[classattr] const DATETIME_YMD_HMS: &'static str = constants::PATTERN_DATETIME_YMD_HMS;

        // @formatter:on
    }

    #[pyclass(name = "token")]
    pub(crate) struct Tokens {}

    #[pymethods]
    impl Tokens {
        // @formatter:off

            // Weekdays
            #[classattr] const WDAY_MON: i16 = constants::TOKEN_WDAY_MON;
            #[classattr] const WDAY_TUE: i16 = constants::TOKEN_WDAY_TUE;
            #[classattr] const WDAY_WED: i16 = constants::TOKEN_WDAY_WED;
            #[classattr] const WDAY_THU: i16 = constants::TOKEN_WDAY_THU;
            #[classattr] const WDAY_FRI: i16 = constants::TOKEN_WDAY_FRI;
            #[classattr] const WDAY_SAT: i16 = constants::TOKEN_WDAY_SAT;
            #[classattr] const WDAY_SUN: i16 = constants::TOKEN_WDAY_SUN;

            // Months
            #[classattr] const MONTH_JAN: i16 = constants::TOKEN_MONTH_JAN;
            #[classattr] const MONTH_FEB: i16 = constants::TOKEN_MONTH_FEB;
            #[classattr] const MONTH_MAR: i16 = constants::TOKEN_MONTH_MAR;
            #[classattr] const MONTH_APR: i16 = constants::TOKEN_MONTH_APR;
            #[classattr] const MONTH_MAY: i16 = constants::TOKEN_MONTH_MAY;
            #[classattr] const MONTH_JUN: i16 = constants::TOKEN_MONTH_JUN;
            #[classattr] const MONTH_JUL: i16 = constants::TOKEN_MONTH_JUL;
            #[classattr] const MONTH_AUG: i16 = constants::TOKEN_MONTH_AUG;
            #[classattr] const MONTH_SEP: i16 = constants::TOKEN_MONTH_SEP;
            #[classattr] const MONTH_OCT: i16 = constants::TOKEN_MONTH_OCT;
            #[classattr] const MONTH_NOV: i16 = constants::TOKEN_MONTH_NOV;
            #[classattr] const MONTH_DEC: i16 = constants::TOKEN_MONTH_DEC;

            #[classattr] const UNIT_SEC: i16 = constants::TOKEN_UNIT_SEC;
            #[classattr] const UNIT_MIN: i16 = constants::TOKEN_UNIT_MIN;
            #[classattr] const UNIT_HRS: i16 = constants::TOKEN_UNIT_HRS;

            #[classattr] const SHORT_UNIT_SEC: i16 = constants::TOKEN_SHORT_UNIT_SEC;
            #[classattr] const SHORT_UNIT_HRS: i16 = constants::TOKEN_SHORT_UNIT_HRS;
            #[classattr] const SHORT_UNIT_DAY: i16 = constants::TOKEN_SHORT_UNIT_DAY;
            #[classattr] const SHORT_UNIT_WEEK: i16 = constants::TOKEN_SHORT_UNIT_WEEK;
            #[classattr] const SHORT_UNIT_MONTH: i16 = constants::TOKEN_SHORT_UNIT_MONTH;
            #[classattr] const SHORT_UNIT_YEAR: i16 = constants::TOKEN_SHORT_UNIT_YEAR;

            #[classattr] const LONG_UNIT_SEC: i16 = constants::TOKEN_LONG_UNIT_SEC;
            #[classattr] const LONG_UNIT_MIN: i16 = constants::TOKEN_LONG_UNIT_MIN;
            #[classattr] const LONG_UNIT_HRS: i16 = constants::TOKEN_LONG_UNIT_HRS;
            #[classattr] const LONG_UNIT_DAY: i16 = constants::TOKEN_LONG_UNIT_DAY;
            #[classattr] const LONG_UNIT_WEEK: i16 = constants::TOKEN_LONG_UNIT_WEEK;
            #[classattr] const LONG_UNIT_MONTH: i16 = constants::TOKEN_LONG_UNIT_MONTH;
            #[classattr] const LONG_UNIT_YEAR: i16 = constants::TOKEN_LONG_UNIT_YEAR;

            // @formatter:on
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
    /// :type weekday_start_mon: bool, optional
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
        today: Option<Py<PyDate>>,
        weekday_start_mon: bool) -> PyResult<NaiveDate> {
        let result = convert_str(
            &source,
            &python::into_date(py, today)?,
            weekday_start_mon,
            read_patterns(module)?,
            read_tokens(module)?,
        );

        match result {
            Some(v) => Ok(v.date_naive()),
            None => Err(PyValueError::new_err(format!(
                "Unable to convert \"{}\" into datetime", source,
            )))
        }
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
    /// :type weekday_start_mon: bool, optional
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
        now: Option<Py<PyDateTime>>,
        weekday_start_mon: bool) -> PyResult<DateTime<FixedOffset>> {
        let result = convert_str(
            &source,
            &python::into_datetime(py, now)?,
            weekday_start_mon,
            read_patterns(module)?,
            read_tokens(module)?,
        );

        match result {
            Some(v) => Ok(v),
            None => Err(PyValueError::new_err(format!(
                "Unable to convert \"{}\" into datetime", source,
            )))
        }
    }

    /// Turn time duration string into seconds
    ///
    /// Only accepts exact time duration strings, such as "1h" rather than
    /// "1 hour ago". Raises a ValueError if anything else than an exact
    /// length of time is provided, or if years or months have been included.
    ///
    /// :param source: Source string
    /// :type str
    /// :raises ValueError
    /// :rtype float
    ///
    #[pyfunction]
    #[pyo3(
        pass_module,
        signature = (source),
        text_signature = "(source: str)"
    )]
    fn to_seconds(
        module: &Bound<'_, PyModule>,
        source: &str) -> PyResult<f64> {
        let result = convert_seconds(
            &source,
            read_patterns(module)?,
            read_tokens(module)?,
        );

        match result {
            Ok(v) => Ok(v),
            Err(e) => Err(PyValueError::new_err(e)),
        }
    }

    #[pymodule_init]
    fn init(module: &Bound<'_, PyModule>) -> PyResult<()> {
        module.add(ATTR_CONFIG, Config {
            patterns: HashMap::new(),
            tokens: HashMap::new(),
        })?;

        Ok(())
    }

    /// Read custom patterns registered to Python module
    fn read_patterns(
        m: &Bound<'_, PyModule>) -> Result<HashMap<String, String>, PyErr> {
        let config = &m
            .as_borrowed()
            .getattr(ATTR_CONFIG)?
            .downcast_into::<Config>()?
            .borrow();

        Ok(config.patterns.to_owned())
    }

    /// Read custom tokens registered to Python module, and return
    /// them as tokens the tokenization (currently) accepts
    fn read_tokens(
        m: &Bound<'_, PyModule>) -> Result<HashMap<String, Token>, PyErr> {
        let config = &m
            .as_borrowed()
            .getattr(ATTR_CONFIG)?
            .downcast_into::<Config>()?
            .borrow();

        let mut result = HashMap::new();

        for (keyword, token_gid) in config.tokens.to_owned() {
            if let Some(token) = gid_into_token(token_gid) {
                result.insert(keyword, token);
            }
        }

        Ok(result)
    }
}

/// Tokenize source string
fn tokenize_str(
    source: &str,
    custom_tokens: HashMap<String, Token>) -> (String, Vec<Token>, Vec<i64>) {
    let (pattern, tokens) = token::tokenize(&source, custom_tokens);
    let values: Vec<i64> = tokens.iter().map(|p| p.value).collect();
    (pattern, tokens, values)
}


/// Tokenize source string and then convert it into a datetime value
fn convert_str(
    source: &str,
    current_time: &DateTime<FixedOffset>,
    first_weekday_mon: bool,
    custom_patterns: HashMap<String, String>,
    custom_tokens: HashMap<String, Token>) -> Option<DateTime<FixedOffset>> {
    let (pattern, _, values) = tokenize_str(&source, custom_tokens);
    fuzzy::convert(&pattern, &values, &current_time, first_weekday_mon, custom_patterns)
}

/// Tokenize source string and then convert it seconds, reflecting exact duration
fn convert_seconds(
    source: &str,
    custom_patterns: HashMap<String, String>,
    custom_tokens: HashMap<String, Token>) -> Result<f64, String> {
    let (pattern, tokens, values) = tokenize_str(&source, custom_tokens);

    if !token::is_time_duration(&pattern) {
        return Err(format!("Unable to convert \"{}\" into seconds", source));
    }

    for token in tokens {
        if token.token.is_unit() && token.value.eq(&7) {
            return Err(String::from("Converting years into seconds is not supported"))
        }

        if token.token.is_unit() && token.value.eq(&6) {
            return Err(String::from("Converting months into seconds is not supported"))
        }
    }

    let current_time = Utc::now().fixed_offset();

    if let Some(from_time) = fuzzy::convert(&pattern, &values, &current_time, true, custom_patterns) {
        let duration: Duration = from_time - current_time;
        return Ok((duration.num_milliseconds() / 1_000) as f64);
    }

    Err(format!("Unable to convert \"{}\" into seconds", source))
}

/// Turn global identifier into corresponding tokenization token
fn gid_into_token(gid: u32) -> Option<Token> {
    if gid.ge(&101) && gid.le(&107) {
        return Option::from(Token {
            token: token::TokenType::Weekday,
            value: (gid - 100) as i64,
        });
    }

    if gid.ge(&201) && gid.le(&212) {
        return Option::from(Token {
            token: token::TokenType::Month,
            value: (gid - 200) as i64,
        });
    }

    if gid.ge(&301) && gid.le(&303) {
        return Option::from(Token {
            token: token::TokenType::Unit,
            value: (gid - 300) as i64,
        });
    }

    if gid.ge(&401) && gid.le(&407) && !gid.eq(&402) {
        return Option::from(Token {
            token: token::TokenType::ShortUnit,
            value: (gid - 400) as i64,
        });
    }

    if gid.ge(&501) && gid.le(&507) {
        return Option::from(Token {
            token: token::TokenType::LongUnit,
            value: (gid - 500) as i64,
        });
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
            ("@1705072948.544", "2024-01-12 15:22:28.544 +00:00"),
            ("2023-01-01", "2023-01-01 00:00:00 +00:00"),
            ("07.02.2023", "2023-02-07 00:00:00 +00:00"),
            ("7.2.2023", "2023-02-07 00:00:00 +00:00"),
            ("2/7/2023", "2023-02-07 00:00:00 +00:00"),
            ("Dec 7 2023", "2023-12-07 00:00:00 +00:00"),
            ("Dec 7th 2023", "2023-12-07 00:00:00 +00:00"),
            ("Dec 7th, 2023", "2023-12-07 00:00:00 +00:00"),
            ("December 7th 2023", "2023-12-07 00:00:00 +00:00"),
            ("7 Dec 2023", "2023-12-07 00:00:00 +00:00"),
            ("7 December 2023", "2023-12-07 00:00:00 +00:00"),
            ("2023-12-07 15:02", "2023-12-07 15:02:00 +00:00"),
            ("2023-12-07 15:02:01", "2023-12-07 15:02:01 +00:00"),
        ];

        let current_time = Utc::now().fixed_offset();

        for (from_string, expect_time) in expect {
            let result_time = convert_str(
                from_string,
                &current_time,
                true,
                HashMap::new(),
                HashMap::new(),
            );
            assert_eq!(result_time.unwrap().to_string(), expect_time.to_string());
        }
    }

    #[test]
    fn test_fixed_day_month() {
        let expect: Vec<(&str, &str, &str)> = vec![
            ("Dec 7", "2024-01-12T15:22:28+02:00", "2024-12-07 00:00:00 +02:00"),
            ("December 7th", "2024-01-12T15:22:28+02:00", "2024-12-07 00:00:00 +02:00"),
            ("7 Dec", "2024-01-12T15:22:28+02:00", "2024-12-07 00:00:00 +02:00"),
        ];

        for (from_string, current_time, expect_time) in expect {
            let current_time = DateTime::parse_from_rfc3339(current_time).unwrap();
            let result_time = convert_str(
                from_string,
                &current_time,
                true,
                HashMap::new(),
                HashMap::new(),
            );
            assert_eq!(result_time.unwrap().to_string(), expect_time.to_string());
        }
    }

    #[test]
    fn test_keywords() {
        assert_convert_from(vec![
            ("now", "2024-01-12T15:22:28+02:00", "2024-01-12 15:22:28 +02:00"),
            ("midnight", "2024-01-12T15:22:28+02:00", "2024-01-12 00:00:00 +02:00"),
            ("yesterday", "2024-01-12T15:22:28+02:00", "2024-01-11 00:00:00 +02:00"),
            ("tomorrow", "2024-01-12T15:22:28+02:00", "2024-01-13 00:00:00 +02:00"),
        ]);
    }

    #[test]
    fn test_month_ranges() {
        assert_convert_from(vec![
            // First
            ("first day of January", "2024-05-12T15:22:28+02:00", "2024-01-01 00:00:00 +02:00"),
            ("first day of this month", "2024-02-12T15:22:28+02:00", "2024-02-01 00:00:00 +02:00"),
            ("first day of prev month", "2024-03-12T15:22:28+02:00", "2024-02-01 00:00:00 +02:00"),
            ("first day of last month", "2024-03-12T15:22:28+02:00", "2024-02-01 00:00:00 +02:00"),
            ("first day of next month", "2024-02-12T15:22:28+02:00", "2024-03-01 00:00:00 +02:00"),

            // Last
            ("last day of February", "2024-05-12T15:22:28+02:00", "2024-02-29 00:00:00 +02:00"),
            ("last day of this month", "2024-02-12T15:22:28+02:00", "2024-02-29 00:00:00 +02:00"),
            ("last day of prev month", "2024-03-12T15:22:28+02:00", "2024-02-29 00:00:00 +02:00"),
            ("last day of last month", "2024-03-12T15:22:28+02:00", "2024-02-29 00:00:00 +02:00"),
            ("last day of next month", "2023-12-12T15:22:28+02:00", "2024-01-31 00:00:00 +02:00"),
        ]);
    }

    #[test]
    fn test_offset_seconds() {
        assert_convert_from(vec![
            ("this second", "2024-01-12T15:22:28+02:00", "2024-01-12 15:22:28 +02:00"),
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
        ]);
    }

    #[test]
    fn test_offset_minutes() {
        assert_convert_from(vec![
            ("this minute", "2024-01-12T15:22:28+02:00", "2024-01-12 15:22:28 +02:00"),
            ("prev minute", "2024-01-12T15:22:28+02:00", "2024-01-12 15:21:28 +02:00"),
            ("last minute", "2024-01-12T15:22:28+02:00", "2024-01-12 15:21:28 +02:00"),
            ("next minute", "2024-01-12T15:22:28+02:00", "2024-01-12 15:23:28 +02:00"),
            ("-1min", "2024-01-12T15:22:28+02:00", "2024-01-12 15:21:28 +02:00"),
            ("-5 minutes", "2024-01-12 15:22:28+02:00", "2024-01-12 15:17:28 +02:00"),
            ("+60min", "2024-01-12T15:22:28+02:00", "2024-01-12 16:22:28 +02:00"),
            ("+60 minutes", "2024-01-12T15:22:28+02:00", "2024-01-12 16:22:28 +02:00"),
            ("1 min ago", "2024-01-12T15:22:28+02:00", "2024-01-12 15:21:28 +02:00"),
            ("5 minutes ago", "2024-01-12 15:22:28+02:00", "2024-01-12 15:17:28 +02:00"),
        ]);
    }

    #[test]
    fn test_offset_hours() {
        assert_convert_from(vec![
            ("this hour", "2024-01-12T15:22:28+02:00", "2024-01-12 15:22:28 +02:00"),
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
        ]);
    }

    #[test]
    fn test_offset_days() {
        assert_convert_from(vec![
            ("this day", "2024-01-12T15:22:28+02:00", "2024-01-12 15:22:28 +02:00"),
            ("prev day", "2024-01-12T15:22:28+02:00", "2024-01-11 15:22:28 +02:00"),
            ("last day", "2024-01-12T15:22:28+02:00", "2024-01-11 15:22:28 +02:00"),
            ("next day", "2024-01-12T15:22:28+02:00", "2024-01-13 15:22:28 +02:00"),
            ("-1d", "2024-01-12T15:22:28+02:00", "2024-01-11 15:22:28 +02:00"),
            ("-1 day", "2024-01-12T15:22:28+02:00", "2024-01-11 15:22:28 +02:00"),
            ("+1d", "2024-01-12T15:22:28+02:00", "2024-01-13 15:22:28 +02:00"),
            ("+30 days", "2024-01-12T15:22:28+02:00", "2024-02-11 15:22:28 +02:00"),
            ("2 days ago", "2024-01-12T15:22:28+02:00", "2024-01-10 15:22:28 +02:00"),
        ]);
    }

    #[test]
    fn test_offset_weekdays() {
        assert_convert_from(vec![
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
        assert_convert_from(vec![
            ("-1w", "2024-01-25T15:22:28+02:00", "2024-01-18 15:22:28 +02:00"),
            ("-2 weeks", "2024-01-25T15:22:28+02:00", "2024-01-11 15:22:28 +02:00"),
            ("+1w", "2024-01-14T14:22:28+02:00", "2024-01-21 14:22:28 +02:00"),
            ("+2 weeks", "2024-01-08T15:22:28+02:00", "2024-01-22 15:22:28 +02:00"),
            ("1 week ago", "2024-01-25T15:22:28+02:00", "2024-01-18 15:22:28 +02:00"),
        ]);
    }

    #[test]
    fn test_offset_weeks_monday() {
        let expect: Vec<(&str, &str, &str)> = vec![
            ("this week", "2024-01-25T15:22:28+02:00", "2024-01-22 15:22:28 +02:00"),
            ("prev week", "2024-01-25T15:22:28+02:00", "2024-01-15 15:22:28 +02:00"),
            ("last week", "2024-01-25T15:22:28+02:00", "2024-01-15 15:22:28 +02:00"),
            ("next week", "2024-01-13 15:22:28+02:00", "2024-01-15 15:22:28 +02:00"),
        ];

        for (from_string, current_time, expect_time) in expect {
            let current_time = DateTime::parse_from_rfc3339(current_time).unwrap();
            let result_time = convert_str(
                from_string,
                &current_time,
                true,
                HashMap::new(),
                HashMap::new(),
            );
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
        ];

        for (from_string, current_time, expect_time) in expect {
            let current_time = DateTime::parse_from_rfc3339(current_time).unwrap();
            let result_time = convert_str(
                from_string,
                &current_time,
                false,
                HashMap::new(),
                HashMap::new(),
            );
            assert_eq!(result_time.unwrap().to_string(), expect_time.to_string())
        }
    }

    #[test]
    fn test_offset_months() {
        assert_convert_from(vec![
            ("this month", "2024-03-12T15:22:28+02:00", "2024-03-12 15:22:28 +02:00"),
            ("prev month", "2024-03-12T15:22:28+02:00", "2024-02-12 15:22:28 +02:00"),
            ("last month", "2024-03-12T15:22:28+02:00", "2024-02-12 15:22:28 +02:00"),
            ("next month", "2024-12-12T15:22:28+02:00", "2025-01-12 15:22:28 +02:00"),
            ("-1m", "2024-03-12T15:22:28+02:00", "2024-02-12 15:22:28 +02:00"),
            ("-1 month", "2024-03-12T15:22:28+02:00", "2024-02-12 15:22:28 +02:00"),
            ("+1m", "2024-03-12T15:22:28+02:00", "2024-04-12 15:22:28 +02:00"),
            ("+13 months", "2023-12-12T15:22:28+02:00", "2025-01-12 15:22:28 +02:00"),
            ("1 month ago", "2024-03-12T15:22:28+02:00", "2024-02-12 15:22:28 +02:00"),

            // Different number of days in each month
            ("-1m", "2022-05-31T15:22:28+02:00", "2022-04-30 15:22:28 +02:00"),
        ]);
    }

    #[test]
    fn test_offset_years() {
        assert_convert_from(vec![
            ("this year", "2024-01-12T15:22:28+02:00", "2024-01-12 15:22:28 +02:00"),
            ("prev year", "2024-01-12T15:22:28+02:00", "2023-01-12 15:22:28 +02:00"),
            ("last year", "2024-01-12T15:22:28+02:00", "2023-01-12 15:22:28 +02:00"),
            ("next year", "2024-01-12T15:22:28+02:00", "2025-01-12 15:22:28 +02:00"),
            ("-1y", "2024-01-12T15:22:28+02:00", "2023-01-12 15:22:28 +02:00"),
            ("-1 year", "2024-01-12T15:22:28+02:00", "2023-01-12 15:22:28 +02:00"),
            ("+1y", "2024-01-12T15:22:28+02:00", "2025-01-12 15:22:28 +02:00"),
            ("+10 years", "2024-01-12T15:22:28+02:00", "2034-01-12 15:22:28 +02:00"),
            ("2 years ago", "2024-01-12T15:22:28+02:00", "2022-01-12 15:22:28 +02:00"),

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
        assert_convert_from(vec![
            ("yesterday midnight", "2024-01-12T15:22:28+02:00", "2024-01-11 00:00:00 +02:00"),
            ("-2d 1h", "2024-05-12T15:22:28+02:00", "2024-05-10 14:22:28 +02:00"),
            ("-2d 1h midnight", "2024-05-12T15:22:28+02:00", "2024-05-10 00:00:00 +02:00"),
            ("first day of Jan last year", "2024-05-12T15:22:28+02:00", "2023-01-01 00:00:00 +02:00"),
            ("last day of Feb last year", "2024-05-12T15:22:28+02:00", "2023-02-28 00:00:00 +02:00"),
        ]);
    }

    #[test]
    fn test_unsupported() {
        let expect: Vec<&str> = vec![
            "",                       // Not parsed
            " ",                      // Nothing to parse
            "+1day",                  // Not recognized
            "0000-01-12 15:22",       // Year invalid
            "1982-04-32",             // Date invalid
            "1982-04-01 15:61",       // Time invalid
            "Feb 29th 2023",          // Day out of range
            "first day of this week", // Not supported
            "first minute of Jan",    // Not supported
        ];

        let current_time = Utc::now().fixed_offset();

        for from_string in expect {
            let result_time = convert_str(
                from_string,
                &current_time,
                true,
                HashMap::new(),
                HashMap::new(),
            );
            assert!(result_time.is_none());
        }
    }

    #[test]
    fn test_to_seconds_some() {
        let expect: Vec<(&str, f64)> = vec![
            ("1 day", 86400.0), ("1d", 86400.0), ("-1 day", -86400.0),
            ("1 hour", 3600.0), ("1h", 3600.0), ("-1 hour", -3600.0),
            ("1d 1h 1min 2s", 90062.0), ("+1d 1h 1min 2s", 90062.0), ("-1d 1h 1min 2s", -90062.0),
            ("1d 1h 1min -2s", 90058.0), ("-1d 1h 1min +2s", -90058.0), ("-1d +1h -1min", -82860.0),
        ];

        for (from_string, expect_value) in expect {
            let result_value = convert_seconds(from_string, HashMap::new(), HashMap::new());
            assert_eq!(result_value.unwrap(), expect_value);
        }
    }

    #[test]
    fn test_to_seconds_none() {
        let expect: Vec<&str> = vec![
            "", "7", "2020-01-07", "last week", "1 hour ago",
            "1y", "+1 year", "-2 years", "1m", "+1 month", "-2 months",
        ];

        for from_string in expect {
            let result_value = convert_seconds(from_string, HashMap::new(), HashMap::new());
            assert!(result_value.is_err());
        }
    }

    #[test]
    fn test_gid_into_token() {
        for value in 101..108 {
            assert_eq!(gid_into_token(value).unwrap(), Token {
                token: token::TokenType::Weekday,
                value: (value - 100) as i64,
            });
        }
        assert!(gid_into_token(100).is_none());
        assert!(gid_into_token(108).is_none());

        for value in 201..213 {
            assert_eq!(gid_into_token(value).unwrap(), Token {
                token: token::TokenType::Month,
                value: (value - 200) as i64,
            });
        }
        assert!(gid_into_token(200).is_none());
        assert!(gid_into_token(213).is_none());

        for value in 301..304 {
            assert_eq!(gid_into_token(value).unwrap(), Token {
                token: token::TokenType::Unit,
                value: (value - 300) as i64,
            });
        }
        assert!(gid_into_token(300).is_none());
        assert!(gid_into_token(304).is_none());

        for value in 401..408 {
            if !value.eq(&402) {
                assert_eq!(gid_into_token(value).unwrap(), Token {
                    token: token::TokenType::ShortUnit,
                    value: (value - 400) as i64,
                });
            }
        }
        assert!(gid_into_token(400).is_none());
        assert!(gid_into_token(408).is_none());

        for value in 501..508 {
            assert_eq!(gid_into_token(value).unwrap(), Token {
                token: token::TokenType::LongUnit,
                value: (value - 500) as i64,
            });
        }
        assert!(gid_into_token(500).is_none());
        assert!(gid_into_token(508).is_none());
    }

    fn assert_convert_from(expect: Vec<(&str, &str, &str)>) {
        for (from_string, current_time, expect_time) in expect {
            let current_time = DateTime::parse_from_rfc3339(current_time).unwrap();
            let result_time = convert_str(
                from_string,
                &current_time,
                false,
                HashMap::new(),
                HashMap::new(),
            );
            assert_eq!(result_time.unwrap().to_string(), expect_time.to_string());
        }
    }
}

