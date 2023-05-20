use std::iter::Peekable;
use std::str::Chars;

use super::Challenge;

pub struct Day08 {
    lines: Vec<String>,
}

impl Challenge for Day08 {
    const DAY: u8 = 8;
    type Part1Solution = usize;
    type Part2Solution = usize;

    fn new(input: &str) -> Self {
        Self {
            lines: input.lines().map(|line| line.to_owned()).collect(),
        }
    }
    fn solve_part1(&self) -> Self::Part1Solution {
        self.lines
            .iter()
            .map(|line| encoding_overhead(line).unwrap())
            .sum()
    }
    fn solve_part2(&self) -> Self::Part2Solution {
        self.lines.iter().map(|line| escape(line).len() - line.len()).sum()
    }
}

type ParseError = String;

fn encoding_overhead(input: &str) -> Result<usize, ParseError> {
    Ok(input.len() - parse(input)?.len())
}

fn parse(input: &str) -> Result<String, ParseError> {
    Parser::new(input).parse()
}

struct Parser<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Parser {
            input: input.chars().peekable(),
        }
    }

    pub fn parse(mut self) -> Result<String, ParseError> {
        self.expect_char('"')?;
        let string = self.parse_string()?;
        self.expect_char('"')?;
        self.expect_end()?;
        Ok(string)
    }

    fn expect_char(&mut self, expected: char) -> Result<char, ParseError> {
        match self.input.next() {
            None => Err(format!("input ended while expecting `{}`", expected)),
            Some(c) => {
                if c == expected {
                    Ok(c)
                } else {
                    Err(format!("expected `{}`, got `{}`", expected, c))
                }
            }
        }
    }

    fn expect_hex_digit(&mut self) -> Result<char, ParseError> {
        match self.input.next() {
            None => Err("input ended while expecting hex digit".into()),
            Some(c) => {
                if c.is_ascii_hexdigit() {
                    Ok(c)
                } else {
                    Err(format!("expected hex digit, got `{}`", c))
                }
            }
        }
    }

    fn expect_end(&mut self) -> Result<(), ParseError> {
        match self.input.next() {
            Some(_) => Err("input did not end after first string".into()),
            None => Ok(()),
        }
    }

    fn parse_string(&mut self) -> Result<String, ParseError> {
        let mut string = String::new();
        loop {
            match *self
                .input
                .peek()
                .ok_or(String::from("input ended while parsing string"))?
            {
                '"' => {
                    break;
                }
                '\\' => {
                    string.push(self.parse_escape_sequence()?);
                }
                _ => {
                    string.push(self.input.next().unwrap());
                }
            }
        }
        Ok(string)
    }

    fn parse_escape_sequence(&mut self) -> Result<char, ParseError> {
        self.expect_char('\\')?;
        let c = self
            .input
            .next()
            .ok_or(String::from("input ended while parsing escape sequence"))?;
        match c {
            '"' | '\\' => Ok(c),
            'x' => {
                let hex_byte = self.parse_hex_byte()?;
                Ok(if hex_byte < 128 {
                    char::from_u32(hex_byte as u32).unwrap()
                } else {
                    '_' // DIRTY HACK! I should really use bytes instead of strings..
                })
            }
            _ => Err(format!("invalid escaped character: `{}`", c)),
        }
    }

    fn parse_hex_byte(&mut self) -> Result<u8, ParseError> {
        let mut hex_string = String::with_capacity(2);
        hex_string.push(self.expect_hex_digit()?);
        hex_string.push(self.expect_hex_digit()?);
        Ok(u8::from_str_radix(&hex_string, 16).unwrap())
    }
}

fn escape(input: &str) -> String {
    let mut output = String::from("\"");
    for c in input.chars() {
        if c == '"' || c == '\\' {
            output.push('\\');
        }
        output.push(c);
    }
    output.push('"');
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert!(parse("something without surrounding quotes").is_err());
        assert!(parse("").is_err());
        assert!(parse("\"").is_err());
        assert!(parse("\"\"and then more text").is_err());
        assert_eq!(parse("\"\"").unwrap(), "");
        assert_eq!(parse("\"abc\"").unwrap(), "abc");

        assert!(parse(r#""\a""#).is_err());
        assert_eq!(parse(r#""\"""#).unwrap(), "\"");
        assert_eq!(parse(r#""\\""#).unwrap(), "\\");
        assert_eq!(parse(r#""\\""#).unwrap(), "\\");
        assert_eq!(parse(r#""\x21""#).unwrap(), "!");

        assert_eq!(
            parse(r#""a bit \\\\ of \" everything\x0ahere""#).unwrap(),
            "a bit \\\\ of \" everything\nhere"
        );

        assert_eq!(
            parse(r#""can't fit in one byte using utf-8: \xAA""#).unwrap(),
            "can't fit in one byte using utf-8: _"
        )
    }

    #[test]
    fn test_encoding_overhead() {
        let strings = [r#""""#, r#""abc""#, r#""aaa\"aaa""#, r#""\x27""#];
        assert_eq!(strings.into_iter().map(|s| s.len()).sum::<usize>(), 23);
        assert_eq!(
            strings
                .into_iter()
                .map(|s| parse(s).unwrap().len())
                .sum::<usize>(),
            11
        );
        assert_eq!(
            strings
                .into_iter()
                .map(|s| encoding_overhead(s).unwrap())
                .sum::<usize>(),
            12
        );
    }

    #[test]
    fn test_extra_case() {
        let cases = [
            (r#""\x66""#, 1),
            (r#""\\x66""#, 4),
            (r#""\\\x66""#, 2),
            (r#""\\\\x66""#, 5),
            (r#""\xa8br\x8bjr\"""#, 7),
        ];

        for (string, expected) in cases.into_iter() {
            assert_eq!(parse(string).unwrap().len(), expected);
        }
    }

    #[test]
    fn test_escape() {
        assert_eq!(escape(""), "\"\"");
        assert_eq!(escape("\"\""), r#""\"\"""#);
        assert_eq!(escape("\"abc\""), r#""\"abc\"""#);
        assert_eq!(escape(r#""aaa\"aaa""#), r#""\"aaa\\\"aaa\"""#);
        assert_eq!(escape("\"\\x27\""), r#""\"\\x27\"""#);
    }

    // #[test]
    // fn test_escape_len() {
    //     assert_eq!(escape("").len(), 2);
    //     assert_eq!(escape("\"\"").len(), );
    //     assert_eq!(escape("\"abc\"").len(), r#""\"abc\"""#);
    //     assert_eq!(escape(r#""aaa\"aaa""#).len(), r#""\"aaa\\\"aaa\"""#);
    //     assert_eq!(escape("\"\\x27\"").len(), r#""\"\\x27\"""#);
    // }
}
