mod convert;
mod fuzzy;
pub mod pattern;
pub mod token;

use crate::token::{Token, UnitNames, UnitGroup, WeekStartDay};
use chrono::{DateTime, Duration, FixedOffset, Utc};
use std::collections::HashMap;

pub struct FuzzyDate {
    current_time: DateTime<FixedOffset>,
    custom_patterns: HashMap<String, String>,
    custom_tokens: HashMap<String, Token>,
    first_weekday: WeekStartDay,
}

impl FuzzyDate {
    pub fn from_now() -> Self {
        Self::from_time(Utc::now().fixed_offset())
    }

    pub fn from_rfc3339(time: &str) -> Self {
        let time = DateTime::parse_from_rfc3339(time).expect("Invalid RFC 3339 time");
        Self::from_time(time)
    }

    pub fn from_time(current_time: DateTime<FixedOffset>) -> Self {
        Self {
            current_time: current_time,
            custom_patterns: HashMap::new(),
            custom_tokens: HashMap::new(),
            first_weekday: WeekStartDay::Monday,
        }
    }

    pub fn set_custom_patterns(mut self, custom: HashMap<String, String>) -> Self {
        self.custom_patterns = custom;
        self
    }

    pub fn set_custom_tokens(mut self, custom: HashMap<String, Token>) -> Self {
        self.custom_tokens = custom;
        self
    }

    pub fn set_first_weekday(mut self, weekday: WeekStartDay) -> Self {
        self.first_weekday = weekday;
        self
    }

    /// Tokenize source string and then convert it into a datetime value
    pub fn to_datetime(&self, source: &str) -> Option<DateTime<FixedOffset>> {
        let (pattern, tokens) = token::tokenize(&source, self.custom_tokens.to_owned());
        fuzzy::convert(
            &pattern,
            tokens,
            &self.current_time,
            self.first_weekday.eq(&WeekStartDay::Monday),
            self.custom_patterns.to_owned(),
        )
    }
}

pub struct FuzzyDuration {
    custom_units: HashMap<String, String>,
    max_unit: String,
    min_unit: String,
    unit_group: UnitGroup,
}

impl FuzzyDuration {
    pub fn new() -> Self {
        Self {
            custom_units: HashMap::new(),
            min_unit: String::new(),
            max_unit: String::new(),
            unit_group: UnitGroup::Default,
        }
    }

    pub fn set_custom_units(mut self, units: HashMap<String, String>) -> Self {
        self.custom_units = units;
        self
    }

    pub fn set_default_units(mut self, group: UnitGroup) -> Self {
        self.unit_group = group;
        self
    }

    pub fn set_min_unit(mut self, unit: &str) -> Self {
        self.min_unit = unit.to_string();
        self
    }

    pub fn set_max_unit(mut self, unit: &str) -> Self {
        self.max_unit = unit.to_string();
        self
    }

    /// Convert number of seconds into a time duration string
    pub fn to_duration(&self, seconds: f64) -> String {
        let mut unit_names = UnitNames::from_name(&self.unit_group);
        unit_names.add_names(self.custom_units.to_owned());

        fuzzy::to_duration(seconds, &unit_names, &self.max_unit, &self.min_unit)
    }
}

pub struct FuzzySeconds {
    custom_patterns: HashMap<String, String>,
    custom_tokens: HashMap<String, Token>,
}

impl FuzzySeconds {
    pub fn new() -> Self {
        Self { custom_patterns: HashMap::new(), custom_tokens: HashMap::new() }
    }

    pub fn set_custom_patterns(mut self, custom: HashMap<String, String>) -> Self {
        self.custom_patterns = custom;
        self
    }

    pub fn set_custom_tokens(mut self, custom: HashMap<String, Token>) -> Self {
        self.custom_tokens = custom;
        self
    }

    /// Tokenize source string and then convert it seconds, reflecting exact duration
    pub fn to_seconds(&self, source: &str) -> Result<f64, String> {
        let (pattern, tokens) = token::tokenize(&source, self.custom_tokens.to_owned());

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

        if let Some(from_time) = fuzzy::convert(&pattern, tokens, &current_time, true, self.custom_patterns.to_owned())
        {
            let duration: Duration = from_time - current_time;
            return Ok((duration.num_milliseconds() / 1_000) as f64);
        }

        Err(format!("Unable to convert \"{}\" into seconds", source))
    }
}
