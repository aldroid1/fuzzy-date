use std::collections::HashMap;

// Boundary characters that always trigger treating
// parsing collected characters into token(s)
const BOUNDARY_CHARS: [&'static str; 6] = [" ", "-", "/", "+", ":", ","];

// Conditional boundary characters, that are boundaries
// when between numbers, but not between characters
const CONDITIONAL_CHARS: [&'static str; 3] = [".", "T", "W"];

// Characters that get muted from the pattern string
const IGNORED_CHARS: [&'static str; 1] = [","];

// Prefix characters before numbers that mean the value should
// be treated as a timestamp
const PREFIX_CHARS_TIMESTAMP: [&'static str; 1] = ["@"];

const STANDARD_TOKENS: [(&'static str, Token); 141] = [
    // Months, abbreviated
    ("jan", Token { token: TokenType::Month, value: 1, zeros: 0 }),
    ("jan.", Token { token: TokenType::Month, value: 1, zeros: 0 }),
    ("feb", Token { token: TokenType::Month, value: 2, zeros: 0 }),
    ("feb.", Token { token: TokenType::Month, value: 2, zeros: 0 }),
    ("mar", Token { token: TokenType::Month, value: 3, zeros: 0 }),
    ("mar.", Token { token: TokenType::Month, value: 3, zeros: 0 }),
    ("apr", Token { token: TokenType::Month, value: 4, zeros: 0 }),
    ("apr.", Token { token: TokenType::Month, value: 4, zeros: 0 }),
    ("jun", Token { token: TokenType::Month, value: 6, zeros: 0 }),
    ("jun.", Token { token: TokenType::Month, value: 6, zeros: 0 }),
    ("jul", Token { token: TokenType::Month, value: 7, zeros: 0 }),
    ("jul.", Token { token: TokenType::Month, value: 7, zeros: 0 }),
    ("aug", Token { token: TokenType::Month, value: 8, zeros: 0 }),
    ("aug.", Token { token: TokenType::Month, value: 8, zeros: 0 }),
    ("sep", Token { token: TokenType::Month, value: 9, zeros: 0 }),
    ("sep.", Token { token: TokenType::Month, value: 9, zeros: 0 }),
    ("sept", Token { token: TokenType::Month, value: 9, zeros: 0 }),
    ("sept.", Token { token: TokenType::Month, value: 9, zeros: 0 }),
    ("oct", Token { token: TokenType::Month, value: 10, zeros: 0 }),
    ("oct.", Token { token: TokenType::Month, value: 10, zeros: 0 }),
    ("nov", Token { token: TokenType::Month, value: 11, zeros: 0 }),
    ("nov.", Token { token: TokenType::Month, value: 11, zeros: 0 }),
    ("dec", Token { token: TokenType::Month, value: 12, zeros: 0 }),
    ("dec.", Token { token: TokenType::Month, value: 12, zeros: 0 }),
    // Months, full
    ("january", Token { token: TokenType::Month, value: 1, zeros: 0 }),
    ("february", Token { token: TokenType::Month, value: 2, zeros: 0 }),
    ("march", Token { token: TokenType::Month, value: 3, zeros: 0 }),
    ("april", Token { token: TokenType::Month, value: 4, zeros: 0 }),
    ("may", Token { token: TokenType::Month, value: 5, zeros: 0 }),
    ("june", Token { token: TokenType::Month, value: 6, zeros: 0 }),
    ("july", Token { token: TokenType::Month, value: 7, zeros: 0 }),
    ("august", Token { token: TokenType::Month, value: 8, zeros: 0 }),
    ("september", Token { token: TokenType::Month, value: 9, zeros: 0 }),
    ("october", Token { token: TokenType::Month, value: 10, zeros: 0 }),
    ("november", Token { token: TokenType::Month, value: 11, zeros: 0 }),
    ("december", Token { token: TokenType::Month, value: 12, zeros: 0 }),
    // Weekdays
    ("mon", Token { token: TokenType::Weekday, value: 1, zeros: 0 }),
    ("monday", Token { token: TokenType::Weekday, value: 1, zeros: 0 }),
    ("tue", Token { token: TokenType::Weekday, value: 2, zeros: 0 }),
    ("tuesday", Token { token: TokenType::Weekday, value: 2, zeros: 0 }),
    ("wed", Token { token: TokenType::Weekday, value: 3, zeros: 0 }),
    ("wednesday", Token { token: TokenType::Weekday, value: 3, zeros: 0 }),
    ("thu", Token { token: TokenType::Weekday, value: 4, zeros: 0 }),
    ("thursday", Token { token: TokenType::Weekday, value: 4, zeros: 0 }),
    ("fri", Token { token: TokenType::Weekday, value: 5, zeros: 0 }),
    ("friday", Token { token: TokenType::Weekday, value: 5, zeros: 0 }),
    ("sat", Token { token: TokenType::Weekday, value: 6, zeros: 0 }),
    ("saturday", Token { token: TokenType::Weekday, value: 6, zeros: 0 }),
    ("sun", Token { token: TokenType::Weekday, value: 7, zeros: 0 }),
    ("sunday", Token { token: TokenType::Weekday, value: 7, zeros: 0 }),
    // Nth
    ("1.", Token { token: TokenType::Nth, value: 1, zeros: 0 }),
    ("2.", Token { token: TokenType::Nth, value: 2, zeros: 0 }),
    ("3.", Token { token: TokenType::Nth, value: 3, zeros: 0 }),
    ("4.", Token { token: TokenType::Nth, value: 4, zeros: 0 }),
    ("5.", Token { token: TokenType::Nth, value: 5, zeros: 0 }),
    ("6.", Token { token: TokenType::Nth, value: 6, zeros: 0 }),
    ("7.", Token { token: TokenType::Nth, value: 7, zeros: 0 }),
    ("8.", Token { token: TokenType::Nth, value: 8, zeros: 0 }),
    ("9.", Token { token: TokenType::Nth, value: 9, zeros: 0 }),
    ("10.", Token { token: TokenType::Nth, value: 10, zeros: 0 }),
    ("11.", Token { token: TokenType::Nth, value: 11, zeros: 0 }),
    ("12.", Token { token: TokenType::Nth, value: 12, zeros: 0 }),
    ("13.", Token { token: TokenType::Nth, value: 13, zeros: 0 }),
    ("14.", Token { token: TokenType::Nth, value: 14, zeros: 0 }),
    ("15.", Token { token: TokenType::Nth, value: 15, zeros: 0 }),
    ("16.", Token { token: TokenType::Nth, value: 16, zeros: 0 }),
    ("17.", Token { token: TokenType::Nth, value: 17, zeros: 0 }),
    ("18.", Token { token: TokenType::Nth, value: 18, zeros: 0 }),
    ("19.", Token { token: TokenType::Nth, value: 19, zeros: 0 }),
    ("20.", Token { token: TokenType::Nth, value: 20, zeros: 0 }),
    ("21.", Token { token: TokenType::Nth, value: 21, zeros: 0 }),
    ("22.", Token { token: TokenType::Nth, value: 22, zeros: 0 }),
    ("23.", Token { token: TokenType::Nth, value: 23, zeros: 0 }),
    ("24.", Token { token: TokenType::Nth, value: 24, zeros: 0 }),
    ("25.", Token { token: TokenType::Nth, value: 25, zeros: 0 }),
    ("26.", Token { token: TokenType::Nth, value: 26, zeros: 0 }),
    ("27.", Token { token: TokenType::Nth, value: 27, zeros: 0 }),
    ("28.", Token { token: TokenType::Nth, value: 28, zeros: 0 }),
    ("29.", Token { token: TokenType::Nth, value: 29, zeros: 0 }),
    ("30.", Token { token: TokenType::Nth, value: 30, zeros: 0 }),
    ("31.", Token { token: TokenType::Nth, value: 31, zeros: 0 }),
    ("1st", Token { token: TokenType::Nth, value: 1, zeros: 0 }),
    ("2nd", Token { token: TokenType::Nth, value: 2, zeros: 0 }),
    ("3rd", Token { token: TokenType::Nth, value: 3, zeros: 0 }),
    ("4th", Token { token: TokenType::Nth, value: 4, zeros: 0 }),
    ("5th", Token { token: TokenType::Nth, value: 5, zeros: 0 }),
    ("6th", Token { token: TokenType::Nth, value: 6, zeros: 0 }),
    ("7th", Token { token: TokenType::Nth, value: 7, zeros: 0 }),
    ("8th", Token { token: TokenType::Nth, value: 8, zeros: 0 }),
    ("9th", Token { token: TokenType::Nth, value: 9, zeros: 0 }),
    ("10th", Token { token: TokenType::Nth, value: 10, zeros: 0 }),
    ("11th", Token { token: TokenType::Nth, value: 11, zeros: 0 }),
    ("12th", Token { token: TokenType::Nth, value: 12, zeros: 0 }),
    ("13th", Token { token: TokenType::Nth, value: 13, zeros: 0 }),
    ("14th", Token { token: TokenType::Nth, value: 14, zeros: 0 }),
    ("15th", Token { token: TokenType::Nth, value: 15, zeros: 0 }),
    ("16th", Token { token: TokenType::Nth, value: 16, zeros: 0 }),
    ("17th", Token { token: TokenType::Nth, value: 17, zeros: 0 }),
    ("18th", Token { token: TokenType::Nth, value: 18, zeros: 0 }),
    ("19th", Token { token: TokenType::Nth, value: 19, zeros: 0 }),
    ("20th", Token { token: TokenType::Nth, value: 20, zeros: 0 }),
    ("21st", Token { token: TokenType::Nth, value: 21, zeros: 0 }),
    ("22nd", Token { token: TokenType::Nth, value: 22, zeros: 0 }),
    ("23rd", Token { token: TokenType::Nth, value: 23, zeros: 0 }),
    ("24th", Token { token: TokenType::Nth, value: 24, zeros: 0 }),
    ("25th", Token { token: TokenType::Nth, value: 25, zeros: 0 }),
    ("26th", Token { token: TokenType::Nth, value: 26, zeros: 0 }),
    ("27th", Token { token: TokenType::Nth, value: 27, zeros: 0 }),
    ("28th", Token { token: TokenType::Nth, value: 28, zeros: 0 }),
    ("29th", Token { token: TokenType::Nth, value: 29, zeros: 0 }),
    ("30th", Token { token: TokenType::Nth, value: 30, zeros: 0 }),
    ("31st", Token { token: TokenType::Nth, value: 31, zeros: 0 }),
    // Time units
    ("sec", Token { token: TokenType::Unit, value: 1, zeros: 0 }),
    ("min", Token { token: TokenType::Unit, value: 2, zeros: 0 }),
    ("mins", Token { token: TokenType::Unit, value: 2, zeros: 0 }),
    ("hr", Token { token: TokenType::Unit, value: 3, zeros: 0 }),
    ("hrs", Token { token: TokenType::Unit, value: 3, zeros: 0 }),
    // Short time units
    ("s", Token { token: TokenType::ShortUnit, value: 1, zeros: 0 }),
    ("h", Token { token: TokenType::ShortUnit, value: 3, zeros: 0 }),
    ("d", Token { token: TokenType::ShortUnit, value: 4, zeros: 0 }),
    ("w", Token { token: TokenType::ShortUnit, value: 5, zeros: 0 }),
    ("m", Token { token: TokenType::ShortUnit, value: 6, zeros: 0 }),
    ("y", Token { token: TokenType::ShortUnit, value: 7, zeros: 0 }),
    // Long time units
    ("second", Token { token: TokenType::LongUnit, value: 1, zeros: 0 }),
    ("seconds", Token { token: TokenType::LongUnit, value: 1, zeros: 0 }),
    ("minute", Token { token: TokenType::LongUnit, value: 2, zeros: 0 }),
    ("minutes", Token { token: TokenType::LongUnit, value: 2, zeros: 0 }),
    ("hour", Token { token: TokenType::LongUnit, value: 3, zeros: 0 }),
    ("hours", Token { token: TokenType::LongUnit, value: 3, zeros: 0 }),
    ("day", Token { token: TokenType::LongUnit, value: 4, zeros: 0 }),
    ("days", Token { token: TokenType::LongUnit, value: 4, zeros: 0 }),
    ("week", Token { token: TokenType::LongUnit, value: 5, zeros: 0 }),
    ("weeks", Token { token: TokenType::LongUnit, value: 5, zeros: 0 }),
    ("month", Token { token: TokenType::LongUnit, value: 6, zeros: 0 }),
    ("months", Token { token: TokenType::LongUnit, value: 6, zeros: 0 }),
    ("year", Token { token: TokenType::LongUnit, value: 7, zeros: 0 }),
    ("years", Token { token: TokenType::LongUnit, value: 7, zeros: 0 }),
    // Meridiems
    ("am", Token { token: TokenType::Meridiem, value: 1, zeros: 0 }),
    ("a.m.", Token { token: TokenType::Meridiem, value: 1, zeros: 0 }),
    ("pm", Token { token: TokenType::Meridiem, value: 2, zeros: 0 }),
    ("p.m.", Token { token: TokenType::Meridiem, value: 2, zeros: 0 }),
];

struct ParsedNumberValue {
    is_timestamp: bool,
    prefix: String,
    number: String,
}

impl ParsedNumberValue {
    fn new(prefix: String, value: String) -> Self {
        Self { is_timestamp: PREFIX_CHARS_TIMESTAMP.contains(&prefix.as_str()), prefix: prefix, number: value }
    }

    fn is_only_number(&self) -> bool {
        (self.prefix.is_empty() || self.is_timestamp) && !self.number.is_empty()
    }

    fn is_only_string(&self) -> bool {
        !self.prefix.is_empty() && self.number.is_empty()
    }

    fn leading_zeros(&self) -> u8 {
        let Some(value) = self.number_value() else { return 0 };

        match value.gt(&0) {
            true => (self.number.len() - self.number.trim_start_matches("0").len()) as u8,
            false => 0,
        }
    }

    fn number_value(&self) -> Option<i64> {
        if self.number.is_empty() {
            return None;
        }

        match self.number.parse::<i64>() {
            Ok(n) => Some(n),
            Err(_) => None,
        }
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub(crate) enum TokenType {
    Integer,
    LongUnit,
    Meridiem,
    Month,
    Nth,
    ShortUnit,
    Timestamp,
    Unit,
    Weekday,
    Year,
}

impl TokenType {
    fn as_name(&self) -> &'static str {
        match self {
            TokenType::Integer => "int",
            TokenType::LongUnit => "long_unit",
            TokenType::Meridiem => "meridiem",
            TokenType::Month => "month",
            TokenType::ShortUnit => "short_unit",
            TokenType::Nth => "nth",
            TokenType::Timestamp => "timestamp",
            TokenType::Unit => "unit",
            TokenType::Weekday => "wday",
            TokenType::Year => "year",
        }
    }

    fn as_pattern(&self) -> String {
        format!("[{}]", self.as_name())
    }

    pub(crate) fn is_unit(&self) -> bool {
        self.eq(&Self::Unit) || self.eq(&Self::ShortUnit) || self.eq(&Self::LongUnit)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Token {
    pub(crate) token: TokenType,
    pub(crate) value: i64,
    pub(crate) zeros: u8,
}

impl Token {
    pub(crate) fn new(token: TokenType, value: i64) -> Self {
        Self { token: token, value: value, zeros: 0 }
    }

    pub(crate) fn new_integer(value: i64, zeros: u8) -> Self {
        Self { token: TokenType::Integer, value: value, zeros: zeros }
    }
}

struct TokenList {
    tokens: HashMap<String, Token>,
}

impl TokenList {
    fn new(custom: HashMap<String, Token>) -> Self {
        let mut tokens = STANDARD_TOKENS
            .iter()
            .map(|(k, t)| (k.to_string(), t.to_owned()))
            .collect::<HashMap<String, Token>>();

        tokens.extend(custom.to_owned());

        Self { tokens: tokens }
    }

    fn find_token(&self, source: &str) -> Option<Token> {
        let lowercased: &str = &source.to_lowercase().to_string();

        match self.tokens.get(lowercased) {
            Some(v) => Some(v.to_owned()),
            None => None,
        }
    }
}

pub(crate) fn is_time_duration(pattern: &str) -> bool {
    let without_integers: String = pattern.replace(TokenType::Integer.as_pattern().as_str(), "");

    if without_integers.eq(&pattern) {
        return false;
    }

    let without_units: String = without_integers
        .replace(TokenType::Unit.as_pattern().as_str(), "")
        .replace(TokenType::ShortUnit.as_pattern().as_str(), "")
        .replace(TokenType::LongUnit.as_pattern().as_str(), "");

    if without_units.eq(&without_integers) {
        return false;
    }

    let without_extra: String = without_units.replace("+", "").replace("-", "").replace(" ", "");

    without_extra.len().eq(&0)
}

/// Turn source string into a pattern, and list of extracted tokens
pub(crate) fn tokenize(source: &str, custom: HashMap<String, Token>) -> (String, Vec<Token>) {
    let mut out_pattern: String = String::new();
    let mut out_values = vec![];

    if source.len().lt(&1) {
        return (out_pattern, out_values);
    }

    let token_list = TokenList::new(custom);
    let last_index: usize = source.len() - 1;
    let mut prev_char = String::new();
    let mut part_start = 0;

    let source_letters = source
        .char_indices()
        .into_iter()
        .map(|v| (v.0, v.1.to_string()))
        .collect::<Vec<(usize, String)>>();

    for (list_index, (part_index, part_char)) in source_letters.iter().enumerate() {
        let mut part_chars = "";
        let mut part_letter: String = String::new();

        let curr_char: &str = &part_char;
        let next_char = source_letters.get(list_index + 1).unwrap_or(&(0, String::new())).1.to_owned();

        if BOUNDARY_CHARS.contains(&curr_char)
            || (CONDITIONAL_CHARS.contains(&curr_char)
                && is_value_boundary(&prev_char, "-")
                && is_value_boundary(&next_char, ""))
        {
            part_chars = &source[part_start..*part_index];
            part_letter.push_str(&curr_char);
            part_start = part_index + 1;
        } else if part_index.eq(&last_index) {
            part_chars = &source[part_start..part_index + 1];
        }

        prev_char = curr_char.to_owned();

        if IGNORED_CHARS.contains(&part_letter.as_str()) {
            part_letter = String::from(" ");
        }

        if part_chars.eq("") {
            if out_values.is_empty() || !&part_letter.eq(" ") {
                out_pattern.push_str(&part_letter);
            }

            continue;
        }

        if let Some(string_value) = token_list.find_token(&part_chars) {
            out_values.push(string_value.clone());
            out_pattern.push_str(&string_value.token.as_pattern());
            out_pattern.push_str(&part_letter);
            continue;
        }

        let parsed_number = parse_string_and_number(part_chars);

        // Just a number, or a special prefix
        if parsed_number.is_only_number() {
            if let Some(number_token) = create_integer_token(&parsed_number) {
                out_values.push(number_token.clone());

                // When timestamp parsing has failed, keep the prefix
                if parsed_number.is_timestamp
                    && number_token.token.ne(&TokenType::Timestamp)
                    && !parsed_number.prefix.is_empty()
                {
                    out_pattern.push_str(&parsed_number.prefix);
                }

                out_pattern.push_str(&number_token.token.as_pattern());
                out_pattern.push_str(&part_letter);
            }
            continue;
        }

        // Unknown string only, include as-is
        if parsed_number.is_only_string() {
            out_pattern.push_str(&part_chars);
            out_pattern.push_str(&part_letter);
            continue;
        }

        let mut combo_pattern = String::new();

        if let Some(number_token) = create_integer_token(&parsed_number) {
            out_values.push(number_token.clone());
            combo_pattern.push_str(&number_token.token.as_pattern());
        } else {
            combo_pattern.push_str(&parsed_number.number);
        }

        if let Some(string_token) = token_list.find_token(&parsed_number.prefix) {
            out_values.push(string_token.clone());
            combo_pattern.push_str(&string_token.token.as_pattern());
        } else {
            combo_pattern.push_str(&parsed_number.prefix);
        }

        out_pattern.push_str(&combo_pattern);
        out_pattern.push_str(&part_letter);
    }

    (out_pattern.trim().to_string(), out_values)
}

/// Check that character is a boundary for value
fn is_value_boundary(prev_char: &String, allow_chars: &str) -> bool {
    prev_char.is_empty() || allow_chars.contains(prev_char) || prev_char.char_indices().nth(0).unwrap().1.is_digit(10)
}

/// Parse a string that consists of a number+string parts, such as "1d"
/// or the supported reverse cases, such as "@123456789
fn parse_string_and_number(part_chars: &str) -> ParsedNumberValue {
    let first_char = part_chars.char_indices().next().unwrap().1.to_string();
    let is_timestamp = ParsedNumberValue::new(first_char, String::new()).is_timestamp;

    let mut curr_number = String::new();
    let mut curr_string = String::new();

    for (_, curr_char) in part_chars.char_indices() {
        if !is_timestamp && curr_string.is_empty() && curr_char.is_digit(10) {
            curr_number.push(curr_char);
            continue;
        }

        if is_timestamp && curr_number.is_empty() && curr_char.is_digit(10) {
            curr_number.push(curr_char);
            continue;
        }

        if is_timestamp && !curr_number.is_empty() {
            curr_number.push(curr_char);
            continue;
        }

        curr_string.push(curr_char);
    }

    ParsedNumberValue::new(curr_string, curr_number)
}

/// Parse a numeric string into an integer token, refining token
/// type based on the size of the integer
fn create_integer_token(parsed: &ParsedNumberValue) -> Option<Token> {
    let Some(number_value) = parsed.number_value() else {
        return None;
    };

    // Only treat large integer values as timestamps when they were
    // parsed with a timestamp prefix character
    if parsed.is_timestamp && number_value.ge(&10_000) {
        return Some(Token::new(TokenType::Timestamp, number_value));
    }

    if number_value.ge(&1_000) && number_value.lt(&10_000) {
        return Some(Token::new(TokenType::Year, number_value));
    }

    Some(Token::new_integer(number_value, parsed.leading_zeros()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_time_duration() {
        let expect: Vec<(&str, bool)> = vec![
            ("[int][short_unit] ", true),
            ("++[int][short_unit]", true),
            ("-[int] [unit]", true),
            ("+[int] [long_unit]", true),
            ("[int] [long_unit] ago", false),
            ("next [long_unit]", false),
        ];

        for (pattern, expect_value) in expect {
            assert_eq!(is_time_duration(pattern), expect_value);
        }
    }

    #[test]
    fn test_weekdays() {
        let expect: Vec<(&str, i64)> = vec![
            ("Monday", 1),
            ("Mon", 1),
            ("Tuesday", 2),
            ("Tue", 2),
            ("Wednesday", 3),
            ("Wed", 3),
            ("Thursday", 4),
            ("Thu", 4),
            ("Friday", 5),
            ("Fri", 5),
            ("Saturday", 6),
            ("Sat", 6),
            ("Sunday", 7),
            ("Sun", 7),
        ];

        for (from_string, expect_value) in expect {
            assert_eq!(
                tokenize_str(from_string),
                (String::from("[wday]"), vec![Token::new(TokenType::Weekday, expect_value)])
            );
        }
    }

    #[test]
    fn test_months_full() {
        let expect: Vec<(&str, i64)> = vec![
            ("January", 1),
            ("February", 2),
            ("March", 3),
            ("April", 4),
            ("May", 5),
            ("June", 6),
            ("July", 7),
            ("August", 8),
            ("September", 9),
            ("October", 10),
            ("November", 11),
            ("December", 12),
        ];

        for (from_string, expect_value) in expect {
            assert_eq!(
                tokenize_str(from_string),
                (String::from("[month]"), vec![Token::new(TokenType::Month, expect_value)])
            );
        }
    }

    #[test]
    fn test_months_abbreviated() {
        let expect: Vec<(&str, i64)> = vec![
            ("Jan", 1),
            ("Feb", 2),
            ("Mar", 3),
            ("Apr", 4),
            ("Jun", 6),
            ("Jun", 6),
            ("Jul", 7),
            ("Aug", 8),
            ("Sep", 9),
            ("Sept", 9),
            ("Oct", 10),
            ("Nov", 11),
            ("Dec", 12),
        ];

        for (from_string, expect_value) in expect {
            assert_eq!(
                tokenize_str(from_string),
                (String::from("[month]"), vec![Token::new(TokenType::Month, expect_value)])
            );

            assert_eq!(
                tokenize_str(&format!("{}.", from_string)),
                (String::from("[month]"), vec![Token::new(TokenType::Month, expect_value)])
            );
        }
    }

    #[test]
    fn test_nth() {
        let expect: Vec<(&str, i64)> = vec![
            ("1st", 1),
            ("2nd", 2),
            ("3rd", 3),
            ("4th", 4),
            ("5th", 5),
            ("6th", 6),
            ("7th", 7),
            ("8th", 8),
            ("9th", 9),
            ("10th", 10),
            ("11th", 11),
            ("12th", 12),
            ("13th", 13),
            ("14th", 14),
            ("15th", 15),
            ("16th", 16),
            ("17th", 17),
            ("18th", 18),
            ("19th", 19),
            ("20th", 20),
            ("21st", 21),
            ("22nd", 22),
            ("23rd", 23),
            ("24th", 24),
            ("25th", 25),
            ("26th", 26),
            ("27th", 27),
            ("28th", 28),
            ("29th", 29),
            ("30th", 30),
            ("31st", 31),
        ];

        for (from_string, expect_value) in expect {
            assert_eq!(
                tokenize_str(from_string),
                (String::from("[nth]"), vec![Token::new(TokenType::Nth, expect_value)])
            );
        }

        for from_day in 1..=31 {
            assert_eq!(
                tokenize_str(&format!("{}. ", from_day)),
                (String::from("[nth]"), vec![Token::new(TokenType::Nth, from_day)])
            );
        }
    }

    #[test]
    fn test_unit() {
        let expect: Vec<(&str, i64)> = vec![("sec", 1), ("min", 2), ("mins", 2), ("hr", 3), ("hrs", 3)];

        for (from_string, expect_value) in expect {
            assert_eq!(
                tokenize_str(format!("-2{}", from_string).as_str()),
                (
                    String::from("-[int][unit]"),
                    vec![
                        Token::new(TokenType::Integer, 2),
                        Token::new(TokenType::Unit, expect_value),
                    ]
                )
            );
        }
    }

    #[test]
    fn test_short_unit() {
        let expect: Vec<(&str, i64)> = vec![("s", 1), ("h", 3), ("d", 4), ("w", 5), ("m", 6), ("y", 7)];

        for (from_string, expect_value) in expect {
            assert_eq!(
                tokenize_str(format!("-2{}", from_string).as_str()),
                (
                    String::from("-[int][short_unit]"),
                    vec![
                        Token::new(TokenType::Integer, 2),
                        Token::new(TokenType::ShortUnit, expect_value)
                    ]
                )
            );
        }
    }

    #[test]
    fn test_long_unit() {
        let expect: Vec<(&str, i64)> = vec![
            ("second", 1),
            ("seconds", 1),
            ("minute", 2),
            ("minutes", 2),
            ("hour", 3),
            ("hours", 3),
            ("day", 4),
            ("days", 4),
            ("week", 5),
            ("weeks", 5),
            ("month", 6),
            ("months", 6),
            ("year", 7),
            ("years", 7),
        ];

        for (from_string, expect_value) in expect {
            assert_eq!(
                tokenize_str(format!("-2{}", from_string).as_str()),
                (
                    String::from("-[int][long_unit]"),
                    vec![
                        Token::new(TokenType::Integer, 2),
                        Token::new(TokenType::LongUnit, expect_value),
                    ]
                )
            );
        }
    }

    #[test]
    fn test_meridiem() {
        let expect: Vec<(&str, i64)> = vec![("am", 1), ("a.m.", 1), ("pm", 2), ("p.m.", 2)];

        for (from_string, expect_value) in expect {
            assert_eq!(
                tokenize_str(format!("2 {}", from_string).as_str()),
                (
                    String::from("[int] [meridiem]"),
                    vec![
                        Token::new(TokenType::Integer, 2),
                        Token::new(TokenType::Meridiem, expect_value),
                    ],
                )
            );

            assert_eq!(
                tokenize_str(format!("2{}", from_string).as_str()),
                (
                    String::from("[int][meridiem]"),
                    vec![
                        Token::new(TokenType::Integer, 2),
                        Token::new(TokenType::Meridiem, expect_value),
                    ],
                )
            );
        }
    }

    #[test]
    fn test_whitespace_ignored() {
        let expect: Vec<(&str, &str)> = vec![
            ("Feb  7th  2023", "[month] [nth] [year]"),
            ("Feb 7th 2023 ", "[month] [nth] [year]"),
            (" 1d  2h 3s", "[int][short_unit] [int][short_unit] [int][short_unit]"),
            ("+1d  -2h 3s", "+[int][short_unit] -[int][short_unit] [int][short_unit]"),
            ("Feb 7th, 2023", "[month] [nth] [year]"),
            (" Feb 7th,  2023", "[month] [nth] [year]"),
            (" Feb 7th,  2023, 12a.m. ", "[month] [nth] [year] [int][meridiem]"),
            (" Feb 7th,  2023, 12  a.m.", "[month] [nth] [year] [int] [meridiem]"),
        ];

        for (from_string, expect_pattern) in expect {
            assert_eq!(tokenize_str(from_string).0, expect_pattern);
        }
    }

    #[test]
    fn test_unit_prefixes() {
        assert_eq!(
            tokenize_str("+1y 5m 2w 5d"),
            (
                String::from("+[int][short_unit] [int][short_unit] [int][short_unit] [int][short_unit]"),
                vec![
                    Token::new(TokenType::Integer, 1),
                    Token::new(TokenType::ShortUnit, 7),
                    Token::new(TokenType::Integer, 5),
                    Token::new(TokenType::ShortUnit, 6),
                    Token::new(TokenType::Integer, 2),
                    Token::new(TokenType::ShortUnit, 5),
                    Token::new(TokenType::Integer, 5),
                    Token::new(TokenType::ShortUnit, 4),
                ]
            )
        );

        assert_eq!(
            tokenize_str("+1y +5m -2w +5d"),
            (
                String::from("+[int][short_unit] +[int][short_unit] -[int][short_unit] +[int][short_unit]"),
                vec![
                    Token::new(TokenType::Integer, 1),
                    Token::new(TokenType::ShortUnit, 7),
                    Token::new(TokenType::Integer, 5),
                    Token::new(TokenType::ShortUnit, 6),
                    Token::new(TokenType::Integer, 2),
                    Token::new(TokenType::ShortUnit, 5),
                    Token::new(TokenType::Integer, 5),
                    Token::new(TokenType::ShortUnit, 4),
                ]
            )
        );

        assert_eq!(
            tokenize_str("+2h 8s"),
            (
                String::from("+[int][short_unit] [int][short_unit]"),
                vec![
                    Token::new(TokenType::Integer, 2),
                    Token::new(TokenType::ShortUnit, 3),
                    Token::new(TokenType::Integer, 8),
                    Token::new(TokenType::ShortUnit, 1),
                ]
            )
        );

        assert_eq!(
            tokenize_str("-2hr 5min 8sec"),
            (
                String::from("-[int][unit] [int][unit] [int][unit]"),
                vec![
                    Token::new(TokenType::Integer, 2),
                    Token::new(TokenType::Unit, 3),
                    Token::new(TokenType::Integer, 5),
                    Token::new(TokenType::Unit, 2),
                    Token::new(TokenType::Integer, 8),
                    Token::new(TokenType::Unit, 1),
                ]
            )
        );

        assert_eq!(
            tokenize_str("-2hrs 5mins 8sec"),
            (
                String::from("-[int][unit] [int][unit] [int][unit]"),
                vec![
                    Token::new(TokenType::Integer, 2),
                    Token::new(TokenType::Unit, 3),
                    Token::new(TokenType::Integer, 5),
                    Token::new(TokenType::Unit, 2),
                    Token::new(TokenType::Integer, 8),
                    Token::new(TokenType::Unit, 1),
                ]
            )
        );
    }

    #[test]
    fn test_nth_dates() {
        assert_eq!(
            tokenize_str("February 7th 2023"),
            (
                String::from("[month] [nth] [year]"),
                vec![
                    Token::new(TokenType::Month, 2),
                    Token::new(TokenType::Nth, 7),
                    Token::new(TokenType::Year, 2023),
                ]
            )
        );

        assert_eq!(
            tokenize_str("February 7. 2023"),
            (
                String::from("[month] [nth] [year]"),
                vec![
                    Token::new(TokenType::Month, 2),
                    Token::new(TokenType::Nth, 7),
                    Token::new(TokenType::Year, 2023),
                ]
            )
        );

        assert_eq!(
            tokenize_str("7. February 2023"),
            (
                String::from("[nth] [month] [year]"),
                vec![
                    Token::new(TokenType::Nth, 7),
                    Token::new(TokenType::Month, 2),
                    Token::new(TokenType::Year, 2023),
                ]
            )
        );
    }

    #[test]
    fn test_timestamps() {
        let expect = vec![
            ("@1705072948", "[timestamp]", vec![Token::new(TokenType::Timestamp, 1705072948)]),
            (
                "@1705072948.0",
                "[timestamp].[int]",
                vec![
                    Token::new(TokenType::Timestamp, 1705072948),
                    Token::new(TokenType::Integer, 0),
                ],
            ),
            ("@1", "@[int]", vec![Token::new(TokenType::Integer, 1)]),
            ("1705072948", "[int]", vec![Token::new(TokenType::Integer, 1705072948)]),
            (
                "1705072948.0",
                "[int].[int]",
                vec![
                    Token::new(TokenType::Integer, 1705072948),
                    Token::new(TokenType::Integer, 0),
                ],
            ),
            ("20120201", "[int]", vec![Token::new(TokenType::Integer, 20120201)]),
        ];

        for (from_string, expect_string, expect_tokens) in expect {
            assert_eq!(tokenize_str(from_string), (expect_string.to_string(), expect_tokens));
        }
    }

    #[test]
    fn test_datetimes() {
        assert_eq!(
            tokenize_str("2023-12-07 15:02"),
            (
                String::from("[year]-[int]-[int] [int]:[int]"),
                vec![
                    Token::new(TokenType::Year, 2023),
                    Token::new_integer(12, 0),
                    Token::new_integer(7, 1),
                    Token::new_integer(15, 0),
                    Token::new_integer(2, 1),
                ]
            )
        );

        assert_eq!(
            tokenize_str("2023-12-07T15:02"),
            (
                String::from("[year]-[int]-[int]T[int]:[int]"),
                vec![
                    Token::new(TokenType::Year, 2023),
                    Token::new_integer(12, 0),
                    Token::new_integer(7, 1),
                    Token::new_integer(15, 0),
                    Token::new_integer(2, 1),
                ]
            )
        );

        assert_eq!(
            tokenize_str("2023-12-07T15:02.100"),
            (
                String::from("[year]-[int]-[int]T[int]:[int].[int]"),
                vec![
                    Token::new(TokenType::Year, 2023),
                    Token::new_integer(12, 0),
                    Token::new_integer(7, 1),
                    Token::new_integer(15, 0),
                    Token::new_integer(2, 1),
                    Token::new_integer(100, 0),
                ]
            )
        );

        assert_eq!(
            tokenize_str("2023-12-07 15:02:01.014"),
            (
                String::from("[year]-[int]-[int] [int]:[int]:[int].[int]"),
                vec![
                    Token::new(TokenType::Year, 2023),
                    Token::new_integer(12, 0),
                    Token::new_integer(7, 1),
                    Token::new_integer(15, 0),
                    Token::new_integer(2, 1),
                    Token::new_integer(1, 1),
                    Token::new_integer(14, 1),
                ]
            )
        );

        assert_eq!(
            tokenize_str("2023-12-07 15:02:01"),
            (
                String::from("[year]-[int]-[int] [int]:[int]:[int]"),
                vec![
                    Token::new(TokenType::Year, 2023),
                    Token::new_integer(12, 0),
                    Token::new_integer(7, 1),
                    Token::new_integer(15, 0),
                    Token::new_integer(2, 1),
                    Token::new_integer(1, 1),
                ]
            )
        );
    }

    #[test]
    fn test_week_numbers() {
        assert_eq!(
            tokenize_str("2025W07"),
            (String::from("[year]W[int]"), vec![Token::new(TokenType::Year, 2025), Token::new_integer(7, 1)])
        );

        assert_eq!(
            tokenize_str("2025-W07"),
            (String::from("[year]-W[int]"), vec![Token::new(TokenType::Year, 2025), Token::new_integer(7, 1)])
        );
    }

    #[test]
    fn test_strings() {
        assert_eq!(
            tokenize_str("2023-07-01"),
            (
                String::from("[year]-[int]-[int]"),
                vec![
                    Token::new(TokenType::Year, 2023),
                    Token::new_integer(7, 1),
                    Token::new_integer(1, 1),
                ]
            )
        );

        assert_eq!(
            tokenize_str("01/07/2023"),
            (
                String::from("[int]/[int]/[year]"),
                vec![
                    Token::new_integer(1, 1),
                    Token::new_integer(7, 1),
                    Token::new(TokenType::Year, 2023),
                ]
            )
        );

        assert_eq!(
            tokenize_str("07.01.2023"),
            (
                String::from("[int].[int].[year]"),
                vec![
                    Token::new_integer(7, 1),
                    Token::new_integer(1, 1),
                    Token::new(TokenType::Year, 2023),
                ]
            )
        );

        assert_eq!(
            tokenize_str("next Monday midnight"),
            (String::from("next [wday] midnight"), vec![Token::new(TokenType::Weekday, 1)])
        );
    }

    #[test]
    fn test_custom_tokens() {
        let custom_tokens = HashMap::from([
            (String::from("maanantai"), Token::new(TokenType::Weekday, 1)),
            (String::from("måndag"), Token::new(TokenType::Weekday, 1)),
            (String::from("heinäkuu"), Token::new(TokenType::Month, 7)),
        ]);

        assert_eq!(
            tokenize("next Maanantai", custom_tokens.to_owned()),
            (String::from("next [wday]"), vec![Token::new(TokenType::Weekday, 1)]),
        );

        assert_eq!(
            tokenize("next Måndag", custom_tokens.to_owned()),
            (String::from("next [wday]"), vec![Token::new(TokenType::Weekday, 1)]),
        );

        assert_eq!(
            tokenize("heinäkuu 10. 2023", custom_tokens.to_owned()),
            (
                String::from("[month] [nth] [year]"),
                vec![
                    Token::new(TokenType::Month, 7),
                    Token::new(TokenType::Nth, 10),
                    Token::new(TokenType::Year, 2023)
                ]
            ),
        );
    }

    #[test]
    fn test_ignored() {
        let expect: Vec<&str> = vec!["", "d1", "@not-a-number", "some word", "+word"];

        for from_string in expect {
            assert_eq!(tokenize_str(from_string), (from_string.to_string(), vec![]));
        }
    }

    fn tokenize_str(source: &str) -> (String, Vec<Token>) {
        tokenize(source, HashMap::new())
    }
}
