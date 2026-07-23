use crate::convert::Change;
use crate::fuzzy::{CallPattern, CallValues, FuzzyDate, Rules, TimeUnit, create_call_sequence, parse_pattern};
use crate::pattern::Pattern;
use crate::token::Token;
use chrono::{DateTime, FixedOffset};
use std::collections::HashMap;

struct RangeValue {
    current_time: DateTime<FixedOffset>,
    start_time: FuzzyDate,
    end_time: FuzzyDate,
    rules: Rules,
}

impl RangeValue {
    fn end_first_of_month(mut self) -> Result<Self, ()> {
        self.end_time = self
            .end_time
            .offset_range_month(TimeUnit::Days, self.end_time.month(), Change::First)?;
        Ok(self)
    }

    fn end_move(mut self, unit: TimeUnit, amount: i64) -> Result<Self, ()> {
        self.end_time = self.end_time.offset_unit_keyword(unit, amount, &self.rules)?;
        Ok(self)
    }

    fn end_last_of_month(mut self) -> Result<Self, ()> {
        self.end_time = self
            .end_time
            .offset_range_month(TimeUnit::Days, self.end_time.month(), Change::Last)?;

        Ok(self)
    }

    fn end_last_of_year(mut self) -> Result<Self, ()> {
        self.end_time = self.end_time.offset_range_month(TimeUnit::Days, 12, Change::Last)?;
        Ok(self)
    }

    fn end_with_current(mut self) -> Result<Self, ()> {
        self.end_time = FuzzyDate::from_time(self.current_time);
        Ok(self)
    }

    fn reset_time_all(mut self) -> Result<Self, ()> {
        self.start_time = self.start_time.time_hms(0, 0, 0, 0)?;
        self.end_time = self.end_time.time_hms(0, 0, 0, 0)?;
        Ok(self)
    }

    fn reset_time_min_sec_ms(mut self) -> Result<Self, ()> {
        self.start_time = self.start_time.time_hms(self.start_time.hour(), 0, 0, 0)?;
        self.end_time = self.end_time.time_hms(self.end_time.hour(), 0, 0, 0)?;
        Ok(self)
    }

    fn reset_time_ms(mut self) -> Result<Self, ()> {
        self.start_time =
            self.start_time
                .time_hms(self.start_time.hour(), self.start_time.minute(), self.start_time.seconds(), 0)?;
        self.end_time =
            self.end_time
                .time_hms(self.end_time.hour(), self.end_time.minute(), self.end_time.seconds(), 0)?;
        Ok(self)
    }

    fn reset_time_sec_ms(mut self) -> Result<Self, ()> {
        self.start_time = self
            .start_time
            .time_hms(self.start_time.hour(), self.start_time.minute(), 0, 0)?;
        self.end_time = self.end_time.time_hms(self.end_time.hour(), self.end_time.minute(), 0, 0)?;
        Ok(self)
    }

    fn start_first_of_month(mut self) -> Result<Self, ()> {
        self.start_time = self
            .start_time
            .offset_range_month(TimeUnit::Days, self.start_time.month(), Change::First)?;
        Ok(self)
    }

    fn start_first_of_year(mut self) -> Result<Self, ()> {
        self.start_time = self.start_time.offset_range_month(TimeUnit::Days, 1, Change::First)?;
        Ok(self)
    }
}

/// Perform conversion against pattern and corresponding token values,
/// relative to given datetime
pub(crate) fn convert_to_range(
    pattern: &str,
    tokens: Vec<Token>,
    current_time: &DateTime<FixedOffset>,
    week_start_mon: bool,
    custom_patterns: HashMap<String, String>,
) -> Option<(DateTime<FixedOffset>, DateTime<FixedOffset>)> {
    let Some(call_sequence) = create_call_sequence(pattern, custom_patterns) else {
        return None;
    };

    if call_sequence.calls.len().ne(&1) {
        return None;
    }

    let Some(ctx_time) = parse_pattern(&call_sequence, tokens.to_owned(), current_time, week_start_mon) else {
        return None;
    };

    let rules = Rules { date_years: false, reset_time: false, week_start_mon: week_start_mon };

    let mut ctx_vals = CallValues::from_tokens(tokens);

    for item in call_sequence.calls {
        ctx_vals.position = item.value_offset;

        let range = RangeValue {
            current_time: current_time.to_owned(),
            start_time: FuzzyDate::from_time(ctx_time.time),
            end_time: FuzzyDate::from_time(ctx_time.time),
            rules: rules.to_owned(),
        };

        if let Ok(v) = parse_call_pattern(item, range, &mut ctx_vals) {
            return Some((v.start_time.time, v.end_time.time));
        }
    }

    None
}

/// Turn call pattern into a range
fn parse_call_pattern(call: CallPattern, r: RangeValue, v: &mut CallValues) -> Result<RangeValue, ()> {
    match call.pattern_type {
        Pattern::Today | Pattern::Tomorrow | Pattern::Yesterday => r.end_move(TimeUnit::Days, 1)?.reset_time_all(),

        Pattern::ThisUnit => match v.get_unit(0) {
            TimeUnit::Minutes => r.end_move(TimeUnit::Minutes, 1)?.reset_time_sec_ms(),
            TimeUnit::Hours => r.end_move(TimeUnit::Hours, 1)?.reset_time_min_sec_ms(),
            TimeUnit::Weeks => r.end_with_current()?.end_move(TimeUnit::Days, 1)?.reset_time_all(),
            TimeUnit::Months => r
                .end_with_current()?
                .end_move(TimeUnit::Days, 1)?
                .start_first_of_month()?
                .reset_time_all(),
            TimeUnit::Years => r
                .end_with_current()?
                .start_first_of_year()?
                .end_move(TimeUnit::Days, 1)?
                .reset_time_all(),
            _ => Err(()),
        },

        Pattern::PrevUnit => match v.get_unit(0) {
            TimeUnit::Minutes => r.end_move(TimeUnit::Minutes, 1)?.reset_time_sec_ms(),
            TimeUnit::Hours => r.end_move(TimeUnit::Hours, 1)?.reset_time_min_sec_ms(),
            TimeUnit::Weeks => r.end_move(TimeUnit::Days, 7)?.reset_time_all(),
            TimeUnit::Months => r
                .start_first_of_month()?
                .end_last_of_month()?
                .end_move(TimeUnit::Days, 1)?
                .reset_time_all(),
            TimeUnit::Years => r
                .start_first_of_year()?
                .end_last_of_year()?
                .end_move(TimeUnit::Days, 1)?
                .reset_time_all(),
            _ => Err(()),
        },

        Pattern::PrevNUnit => match v.get_unit(1) {
            TimeUnit::Seconds => r.end_with_current()?.end_move(TimeUnit::Seconds, 1)?.reset_time_ms(),
            TimeUnit::Minutes => r.end_with_current()?.end_move(TimeUnit::Minutes, 1)?.reset_time_sec_ms(),
            TimeUnit::Hours => r.end_move(TimeUnit::Hours, v.get_int(0))?.reset_time_min_sec_ms(),
            TimeUnit::Days => r.end_move(TimeUnit::Days, v.get_int(0))?.reset_time_all(),
            TimeUnit::Weeks => r.end_move(TimeUnit::Days, v.get_int(0) * 7)?.reset_time_all(),
            TimeUnit::Months => r
                .start_first_of_month()?
                .end_move(TimeUnit::Months, v.get_int(0))?
                .end_first_of_month()?
                .reset_time_all(),
            TimeUnit::Years => r
                .start_first_of_year()?
                .end_first_of_month()?
                .end_move(TimeUnit::Years, v.get_int(0) - 1)?
                .end_last_of_year()?
                .end_move(TimeUnit::Days, 1)?
                .reset_time_all(),
            _ => Err(()),
        },

        Pattern::NextUnit => match v.get_unit(0) {
            TimeUnit::Minutes => r.end_move(TimeUnit::Minutes, 1)?.reset_time_sec_ms(),
            TimeUnit::Hours => r.end_move(TimeUnit::Hours, 1)?.reset_time_min_sec_ms(),
            TimeUnit::Weeks => r.end_move(TimeUnit::Days, 7)?.reset_time_all(),
            TimeUnit::Months => r
                .start_first_of_month()?
                .end_last_of_month()?
                .end_move(TimeUnit::Days, 1)?
                .reset_time_all(),
            TimeUnit::Years => r
                .start_first_of_year()?
                .end_last_of_year()?
                .end_move(TimeUnit::Days, 1)?
                .reset_time_all(),
            _ => Err(()),
        },

        _ => Err(()),
    }
}
