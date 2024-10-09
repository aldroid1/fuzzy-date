use std::collections::{HashMap, HashSet};

const BOUNDARY_CHARS: [&'static str; 6] = [
    " ", "-", "/", "+", ":", ".",
];

const STANDARD_TOKENS: [(&'static str, Token); 93] = [
    // Months
    ("jan", Token { token: TokenType::Month, value: 1 }),
    ("january", Token { token: TokenType::Month, value: 1 }),
    ("feb", Token { token: TokenType::Month, value: 2 }),
    ("february", Token { token: TokenType::Month, value: 2 }),
    ("mar", Token { token: TokenType::Month, value: 3 }),
    ("march", Token { token: TokenType::Month, value: 3 }),
    ("apr", Token { token: TokenType::Month, value: 4 }),
    ("april", Token { token: TokenType::Month, value: 4 }),
    ("may", Token { token: TokenType::Month, value: 5 }),
    ("jun", Token { token: TokenType::Month, value: 6 }),
    ("june", Token { token: TokenType::Month, value: 6 }),
    ("jul", Token { token: TokenType::Month, value: 7 }),
    ("july", Token { token: TokenType::Month, value: 7 }),
    ("aug", Token { token: TokenType::Month, value: 8 }),
    ("august", Token { token: TokenType::Month, value: 8 }),
    ("sep", Token { token: TokenType::Month, value: 9 }),
    ("september", Token { token: TokenType::Month, value: 9 }),
    ("oct", Token { token: TokenType::Month, value: 10 }),
    ("october", Token { token: TokenType::Month, value: 10 }),
    ("nov", Token { token: TokenType::Month, value: 11 }),
    ("november", Token { token: TokenType::Month, value: 11 }),
    ("dec", Token { token: TokenType::Month, value: 12 }),
    ("december", Token { token: TokenType::Month, value: 12 }),

    // Weekdays
    ("mon", Token { token: TokenType::Weekday, value: 1 }),
    ("monday", Token { token: TokenType::Weekday, value: 1 }),
    ("tue", Token { token: TokenType::Weekday, value: 2 }),
    ("tuesday", Token { token: TokenType::Weekday, value: 2 }),
    ("wed", Token { token: TokenType::Weekday, value: 3 }),
    ("wednesday", Token { token: TokenType::Weekday, value: 3 }),
    ("thu", Token { token: TokenType::Weekday, value: 4 }),
    ("thursday", Token { token: TokenType::Weekday, value: 4 }),
    ("fri", Token { token: TokenType::Weekday, value: 5 }),
    ("friday", Token { token: TokenType::Weekday, value: 5 }),
    ("sat", Token { token: TokenType::Weekday, value: 6 }),
    ("saturday", Token { token: TokenType::Weekday, value: 6 }),
    ("sun", Token { token: TokenType::Weekday, value: 7 }),
    ("sunday", Token { token: TokenType::Weekday, value: 7 }),

    // Nth
    ("1st", Token { token: TokenType::Nth, value: 1 }),
    ("2nd", Token { token: TokenType::Nth, value: 2 }),
    ("3rd", Token { token: TokenType::Nth, value: 3 }),
    ("4th", Token { token: TokenType::Nth, value: 4 }),
    ("5th", Token { token: TokenType::Nth, value: 5 }),
    ("6th", Token { token: TokenType::Nth, value: 6 }),
    ("7th", Token { token: TokenType::Nth, value: 7 }),
    ("8th", Token { token: TokenType::Nth, value: 8 }),
    ("9th", Token { token: TokenType::Nth, value: 9 }),
    ("10th", Token { token: TokenType::Nth, value: 10 }),
    ("11th", Token { token: TokenType::Nth, value: 11 }),
    ("12th", Token { token: TokenType::Nth, value: 12 }),
    ("13th", Token { token: TokenType::Nth, value: 13 }),
    ("14th", Token { token: TokenType::Nth, value: 14 }),
    ("15th", Token { token: TokenType::Nth, value: 15 }),
    ("16th", Token { token: TokenType::Nth, value: 16 }),
    ("17th", Token { token: TokenType::Nth, value: 17 }),
    ("18th", Token { token: TokenType::Nth, value: 18 }),
    ("19th", Token { token: TokenType::Nth, value: 19 }),
    ("20th", Token { token: TokenType::Nth, value: 20 }),
    ("21st", Token { token: TokenType::Nth, value: 21 }),
    ("22nd", Token { token: TokenType::Nth, value: 22 }),
    ("23rd", Token { token: TokenType::Nth, value: 23 }),
    ("24th", Token { token: TokenType::Nth, value: 24 }),
    ("25th", Token { token: TokenType::Nth, value: 25 }),
    ("26th", Token { token: TokenType::Nth, value: 26 }),
    ("27th", Token { token: TokenType::Nth, value: 27 }),
    ("28th", Token { token: TokenType::Nth, value: 28 }),
    ("29th", Token { token: TokenType::Nth, value: 29 }),
    ("30th", Token { token: TokenType::Nth, value: 30 }),
    ("31st", Token { token: TokenType::Nth, value: 31 }),

    // Time units
    ("sec", Token { token: TokenType::Unit, value: 1 }),
    ("min", Token { token: TokenType::Unit, value: 2 }),
    ("mins", Token { token: TokenType::Unit, value: 2 }),
    ("hr", Token { token: TokenType::Unit, value: 3 }),
    ("hrs", Token { token: TokenType::Unit, value: 3 }),

    // Short time units
    ("s", Token { token: TokenType::ShortUnit, value: 1 }),
    ("h", Token { token: TokenType::ShortUnit, value: 3 }),
    ("d", Token { token: TokenType::ShortUnit, value: 4 }),
    ("w", Token { token: TokenType::ShortUnit, value: 5 }),
    ("m", Token { token: TokenType::ShortUnit, value: 6 }),
    ("y", Token { token: TokenType::ShortUnit, value: 7 }),

    // Long time units
    ("second", Token { token: TokenType::LongUnit, value: 1 }),
    ("seconds", Token { token: TokenType::LongUnit, value: 1 }),
    ("minute", Token { token: TokenType::LongUnit, value: 2 }),
    ("minutes", Token { token: TokenType::LongUnit, value: 2 }),
    ("hour", Token { token: TokenType::LongUnit, value: 3 }),
    ("hours", Token { token: TokenType::LongUnit, value: 3 }),
    ("day", Token { token: TokenType::LongUnit, value: 4 }),
    ("days", Token { token: TokenType::LongUnit, value: 4 }),
    ("week", Token { token: TokenType::LongUnit, value: 5 }),
    ("weeks", Token { token: TokenType::LongUnit, value: 5 }),
    ("month", Token { token: TokenType::LongUnit, value: 6 }),
    ("months", Token { token: TokenType::LongUnit, value: 6 }),
    ("year", Token { token: TokenType::LongUnit, value: 7 }),
    ("years", Token { token: TokenType::LongUnit, value: 7 }),
];

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
enum TokenType {
    Integer,
    LongUnit,
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
            TokenType::Month => "month",
            TokenType::ShortUnit => "short_unit",
            TokenType::Nth => "nth",
            TokenType::Timestamp => "timestamp",
            TokenType::Unit => "unit",
            TokenType::Weekday => "wday",
            TokenType::Year => "year",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Token {
    token: TokenType,
    pub(crate) value: i64,
}

impl Token {
    fn name(&self) -> String {
        format!("[{}]", self.token.as_name())
    }
}

struct TokenList {}

impl TokenList {
    fn is_boundary(value: &str) -> bool {
        let boundary_chars = HashSet::from(BOUNDARY_CHARS);
        boundary_chars.contains(value)
    }

    fn is_prefixed(value: &str) -> bool {
        HashSet::from(["@"]).contains(value)
    }

    fn find_token(source: &str) -> Option<Token> {
        let token_strings = HashMap::from(STANDARD_TOKENS);
        let lowercased: &str = &source.to_lowercase().to_string();

        match token_strings.get(lowercased) {
            Some(v) => Option::from(v.clone()),
            None => None,
        }
    }
}

/// Turn source string into a pattern, and list of extracted tokens
pub(crate) fn tokenize(source: &str) -> (String, Vec<Token>) {
    let mut out_pattern: String = String::from("");
    let mut out_values = vec![];

    if source.len().lt(&1) {
        return (out_pattern, out_values);
    }

    let last_index: usize = source.len() - 1;
    let mut part_start = 0;

    for (part_index, part_char) in source.chars().enumerate() {
        let mut part_chars = "";
        let mut part_letter: String = String::from("");

        let char_str: &str = &part_char.to_string();

        if TokenList::is_boundary(&char_str) {
            part_chars = &source[part_start..part_index];
            part_letter.push_str(&char_str);
            part_start = part_index + 1;
        } else if part_index.eq(&last_index) {
            part_chars = &source[part_start..part_index + 1];
        }

        if part_chars.eq("") {
            if out_values.len() == 0 {
                out_pattern.push_str(&part_letter);
            }

            continue;
        }

        let string_token = TokenList::find_token(&part_chars);

        if string_token.is_some() {
            let string_value = string_token.unwrap();
            out_values.push(string_value.clone());
            out_pattern.push_str(&string_value.name());
            out_pattern.push_str(&part_letter);
            continue;
        }

        let (curr_string, curr_number) = parse_string_and_number(part_chars);
        let number_value = parse_number(&curr_number);

        // Just a number, or a special prefix
        if curr_number.len() > 0 && curr_string.len() == 0 {
            if number_value.is_some() {
                let number_token = number_value.unwrap();
                out_values.push(number_token.clone());
                out_pattern.push_str(&number_token.name());
                out_pattern.push_str(&part_letter);
            }
            continue;
        }

        // Unknown string only, include as-is
        if curr_number.len() == 0 && curr_string.len() > 0 {
            out_pattern.push_str(&part_chars);
            out_pattern.push_str(&part_letter);
            continue;
        }

        let mut combo_pattern = String::from("");

        if number_value.is_some() {
            let number_token = number_value.unwrap();
            out_values.push(number_token.clone());
            combo_pattern.push_str(&number_token.name());
        } else {
            combo_pattern.push_str(&curr_number);
        }

        let string_value = TokenList::find_token(&curr_string);

        if string_value.is_some() {
            let string_token = string_value.unwrap();
            out_values.push(string_token.clone());
            combo_pattern.push_str(&string_token.name());
        } else {
            combo_pattern.push_str(&curr_string);
        }

        out_pattern.push_str(&combo_pattern);
        out_pattern.push_str(&part_letter);
    }

    (out_pattern.trim_end().to_string(), out_values)
}

/// Parse a string that consists of a number+string parts, such as "1d"
/// or the supported reverse cases, such as "@123456789
fn parse_string_and_number(part_chars: &str) -> (String, String) {
    let prefixed_number: bool = TokenList::is_prefixed(
        &part_chars.chars().next().unwrap().to_string().as_str()
    );

    let mut curr_number = String::from("");
    let mut curr_string = String::from("");

    for curr_char in part_chars.chars() {
        if prefixed_number.eq(&false) && curr_string.len() == 0 && curr_char.is_digit(10) {
            curr_number.push(curr_char);
            continue;
        }

        if prefixed_number.eq(&true) && curr_number.len() == 0 && curr_char.is_digit(10) {
            curr_number.push(curr_char);
            continue;
        }

        if prefixed_number.eq(&true) && curr_number.len() > 0 {
            curr_number.push(curr_char);
            continue;
        }

        curr_string.push(curr_char);
    }

    if prefixed_number.eq(&true)
        && curr_number.len() > 0
        && TokenList::is_prefixed(curr_string.as_str()) {
        curr_string = "".to_string();
    }

    (curr_string, curr_number)
}

/// Parse a numeric string into an integer token, refining token
/// type based on the size of the integer
fn parse_number(source: &str) -> Option<Token> {
    if source.len() == 0 {
        return None;
    }

    let value: i64 = match source.parse::<i64>() {
        Ok(v) => v,
        Err(_) => return None,
    };

    if value.ge(&10000) {
        return Option::from(Token {
            token: TokenType::Timestamp,
            value: value,
        });
    }

    if value.ge(&1000) {
        return Option::from(Token {
            token: TokenType::Year,
            value: value,
        });
    }

    Option::from(Token {
        token: TokenType::Integer,
        value: value,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weekdays() {
        let expect: Vec<(&str, i64)> = vec![
            ("Monday", 1), ("Mon", 1), ("Tuesday", 2), ("Tue", 2),
            ("Wednesday", 3), ("Wed", 3), ("Thursday", 4), ("Thu", 4),
            ("Friday", 5), ("Fri", 5), ("Saturday", 6), ("Sat", 6),
            ("Sunday", 7), ("Sun", 7),
        ];

        for (from_string, expect_value) in expect {
            assert_eq!(tokenize(from_string), (
                String::from("[wday]"),
                vec![Token { token: TokenType::Weekday, value: expect_value }]
            ));
        }
    }

    #[test]
    fn test_months() {
        let expect: Vec<(&str, i64)> = vec![
            ("January", 1), ("Jan", 1), ("February", 2), ("Feb", 2),
            ("March", 3), ("Mar", 3), ("April", 4), ("Apr", 4),
            ("May", 5), ("June", 6), ("Jun", 6),
            ("July", 7), ("Jul", 7), ("August", 8), ("Aug", 8),
            ("September", 9), ("Sep", 9), ("October", 10), ("Oct", 10),
            ("November", 11), ("Nov", 11), ("December", 12), ("Dec", 12),
        ];

        for (from_string, expect_value) in expect {
            assert_eq!(tokenize(from_string), (
                String::from("[month]"),
                vec![Token { token: TokenType::Month, value: expect_value }]
            ));
        }
    }

    #[test]
    fn test_nth() {
        let expect: Vec<(&str, i64)> = vec![
            ("1st", 1), ("2nd", 2), ("3rd", 3), ("4th", 4), ("5th", 5),
            ("6th", 6), ("7th", 7), ("8th", 8), ("9th", 9), ("10th", 10),
            ("11th", 11), ("12th", 12), ("13th", 13), ("14th", 14), ("15th", 15),
            ("16th", 16), ("17th", 17), ("18th", 18), ("19th", 19), ("20th", 20),
            ("21st", 21), ("22nd", 22), ("23rd", 23), ("24th", 24), ("25th", 25),
            ("26th", 26), ("27th", 27), ("28th", 28), ("29th", 29), ("30th", 30),
            ("31st", 31),
        ];

        for (from_string, expect_value) in expect {
            assert_eq!(tokenize(from_string), (
                String::from("[nth]"),
                vec![Token { token: TokenType::Nth, value: expect_value }]
            ));
        }
    }

    #[test]
    fn test_unit() {
        let expect: Vec<(&str, i64)> = vec![
            ("sec", 1), ("min", 2), ("mins", 2), ("hr", 3), ("hrs", 3),
        ];

        for (from_string, expect_value) in expect {
            assert_eq!(tokenize(format!("-2{}", from_string).as_str()), (
                String::from("-[int][unit]"),
                vec![
                    Token { token: TokenType::Integer, value: 2 },
                    Token { token: TokenType::Unit, value: expect_value }
                ]
            ));
        }
    }

    #[test]
    fn test_short_unit() {
        let expect: Vec<(&str, i64)> = vec![
            ("s", 1), ("h", 3), ("d", 4), ("w", 5), ("m", 6), ("y", 7),
        ];

        for (from_string, expect_value) in expect {
            assert_eq!(tokenize(format!("-2{}", from_string).as_str()), (
                String::from("-[int][short_unit]"),
                vec![
                    Token { token: TokenType::Integer, value: 2 },
                    Token { token: TokenType::ShortUnit, value: expect_value }
                ]
            ));
        }
    }

    #[test]
    fn test_long_unit() {
        let expect: Vec<(&str, i64)> = vec![
            ("second", 1), ("seconds", 1), ("minute", 2), ("minutes", 2),
            ("hour", 3), ("hours", 3), ("day", 4), ("days", 4),
            ("week", 5), ("weeks", 5), ("month", 6), ("months", 6),
            ("year", 7), ("years", 7),
        ];

        for (from_string, expect_value) in expect {
            assert_eq!(tokenize(format!("-2{}", from_string).as_str()), (
                String::from("-[int][long_unit]"),
                vec![
                    Token { token: TokenType::Integer, value: 2 },
                    Token { token: TokenType::LongUnit, value: expect_value }
                ]
            ));
        }
    }

    #[test]
    fn test_whitespace() {
        let expect: Vec<(&str, &str, (i64, i64, i64))> = vec![
            ("Feb  7th  2023", "[month] [nth] [year]", (2, 7, 2023)),
            ("Feb 7th 2023 ", "[month] [nth] [year]", (2, 7, 2023)),
        ];

        for (from_string, expect_pattern, expect_values) in expect {
            assert_eq!(tokenize(from_string), (
                expect_pattern.to_string(),
                vec![
                    Token { token: TokenType::Month, value: expect_values.0 },
                    Token { token: TokenType::Nth, value: expect_values.1 },
                    Token { token: TokenType::Year, value: expect_values.2 },
                ]
            ));
        }
    }

    #[test]
    fn test_strings() {
        assert_eq!(
            tokenize("@1705072948"), (
                String::from("[timestamp]"),
                vec![
                    Token { token: TokenType::Timestamp, value: 1705072948 },
                ]
            )
        );

        assert_eq!(
            tokenize("@1705072948.0"), (
                String::from("[timestamp].[int]"),
                vec![
                    Token { token: TokenType::Timestamp, value: 1705072948 },
                    Token { token: TokenType::Integer, value: 0 },
                ]
            )
        );

        assert_eq!(
            tokenize("2023-07-01"), (
                String::from("[year]-[int]-[int]"),
                vec![
                    Token { token: TokenType::Year, value: 2023 },
                    Token { token: TokenType::Integer, value: 7 },
                    Token { token: TokenType::Integer, value: 1 },
                ]
            )
        );

        assert_eq!(
            tokenize("2023-12-07 15:02"), (
                String::from("[year]-[int]-[int] [int]:[int]"),
                vec![
                    Token { token: TokenType::Year, value: 2023 },
                    Token { token: TokenType::Integer, value: 12 },
                    Token { token: TokenType::Integer, value: 7 },
                    Token { token: TokenType::Integer, value: 15 },
                    Token { token: TokenType::Integer, value: 2 },
                ]
            )
        );

        assert_eq!(
            tokenize("2023-12-07 15:02:01"), (
                String::from("[year]-[int]-[int] [int]:[int]:[int]"),
                vec![
                    Token { token: TokenType::Year, value: 2023 },
                    Token { token: TokenType::Integer, value: 12 },
                    Token { token: TokenType::Integer, value: 7 },
                    Token { token: TokenType::Integer, value: 15 },
                    Token { token: TokenType::Integer, value: 2 },
                    Token { token: TokenType::Integer, value: 1 },
                ]
            )
        );

        assert_eq!(
            tokenize("01/07/2023"), (
                String::from("[int]/[int]/[year]"),
                vec![
                    Token { token: TokenType::Integer, value: 1 },
                    Token { token: TokenType::Integer, value: 7 },
                    Token { token: TokenType::Year, value: 2023 },
                ]
            )
        );

        assert_eq!(
            tokenize("07.01.2023"), (
                String::from("[int].[int].[year]"),
                vec![
                    Token { token: TokenType::Integer, value: 7 },
                    Token { token: TokenType::Integer, value: 1 },
                    Token { token: TokenType::Year, value: 2023 },
                ]
            )
        );

        assert_eq!(
            tokenize("February 7th 2023"), (
                String::from("[month] [nth] [year]"),
                vec![
                    Token { token: TokenType::Month, value: 2 },
                    Token { token: TokenType::Nth, value: 7 },
                    Token { token: TokenType::Year, value: 2023 },
                ]
            )
        );

        assert_eq!(
            tokenize("+1y 5m 2w 5d"), (
                String::from("+[int][short_unit] [int][short_unit] [int][short_unit] [int][short_unit]"),
                vec![
                    Token { token: TokenType::Integer, value: 1 },
                    Token { token: TokenType::ShortUnit, value: 7 },
                    Token { token: TokenType::Integer, value: 5 },
                    Token { token: TokenType::ShortUnit, value: 6 },
                    Token { token: TokenType::Integer, value: 2 },
                    Token { token: TokenType::ShortUnit, value: 5 },
                    Token { token: TokenType::Integer, value: 5 },
                    Token { token: TokenType::ShortUnit, value: 4 },
                ]
            )
        );

        assert_eq!(
            tokenize("+2h 8s"), (
                String::from("+[int][short_unit] [int][short_unit]"),
                vec![
                    Token { token: TokenType::Integer, value: 2 },
                    Token { token: TokenType::ShortUnit, value: 3 },
                    Token { token: TokenType::Integer, value: 8 },
                    Token { token: TokenType::ShortUnit, value: 1 },
                ]
            )
        );

        assert_eq!(
            tokenize("-2hr 5min 8sec"), (
                String::from("-[int][unit] [int][unit] [int][unit]"),
                vec![
                    Token { token: TokenType::Integer, value: 2 },
                    Token { token: TokenType::Unit, value: 3 },
                    Token { token: TokenType::Integer, value: 5 },
                    Token { token: TokenType::Unit, value: 2 },
                    Token { token: TokenType::Integer, value: 8 },
                    Token { token: TokenType::Unit, value: 1 },
                ]
            )
        );

        assert_eq!(
            tokenize("-2hrs 5mins 8sec"), (
                String::from("-[int][unit] [int][unit] [int][unit]"),
                vec![
                    Token { token: TokenType::Integer, value: 2 },
                    Token { token: TokenType::Unit, value: 3 },
                    Token { token: TokenType::Integer, value: 5 },
                    Token { token: TokenType::Unit, value: 2 },
                    Token { token: TokenType::Integer, value: 8 },
                    Token { token: TokenType::Unit, value: 1 },
                ]
            )
        );

        assert_eq!(
            tokenize("next Monday midnight"), (
                String::from("next [wday] midnight"),
                vec![
                    Token { token: TokenType::Weekday, value: 1 },
                ]
            )
        );
    }

    #[test]
    fn test_ignored() {
        let expect: Vec<&str> = vec![
            "", "d1", "@not-a-number", "some word", "+word",
        ];

        for from_string in expect {
            assert_eq!(tokenize(from_string), (from_string.to_string(), vec![]));
        }
    }
}