use crate::Result;
use crate::{take_char_while, take_while, GrammarUnit, ParseError};

#[derive(Debug, PartialEq)]
pub struct QuotedString {
    pub value: String,
    start: usize,
    end: usize,
}

#[derive(Debug, PartialEq)]
pub struct BareWord {
    start: usize,
    end: usize,
}

#[derive(Debug, PartialEq)]
pub struct Whitespace {
    start: usize,
    end: usize,
}

#[derive(Debug, PartialEq)]
pub struct Numeric {
    pub value: f64,
    start: usize,
    end: usize,
}

#[derive(Debug, PartialEq)]
pub struct Integer {
    pub value: i64,
    start: usize,
    end: usize,
}

impl GrammarUnit for QuotedString {
    fn try_match(content: &str, offset: usize) -> Result<(Self, usize)> {
        if !content.starts_with('"') {
            return Err(ParseError::new(
                String::from("expected quoted string"),
                offset,
                offset + 1,
            ));
        }

        let inside_start = 1;
        let inside_size = take_while(&content[inside_start..], |c| {
            if c.starts_with("\\\"") {
                return 2;
            }
            if c.starts_with('"') {
                return 0;
            }
            c.chars().next().map(|ch| ch.len_utf8()).unwrap_or(0)
        });

        let inside_end = inside_start + inside_size;
        let end = inside_end + 1;

        let last = &content[inside_end..].chars().next();

        match last {
            Some('"') => (),
            Some(_) | None => {
                return Err(ParseError::new(
                    String::from("unterminated quoted string"),
                    offset,
                    offset + end,
                ));
            }
        }

        let value = content[inside_start..inside_end].replace("\\\"", "\"");

        Ok((
            QuotedString {
                value,
                start: offset,
                end: end + offset,
            },
            end,
        ))
    }

    fn range(&self) -> (usize, usize) {
        (self.start, self.end)
    }
}

impl GrammarUnit for Whitespace {
    fn try_match(content: &str, offset: usize) -> Result<(Self, usize)> {
        let size = take_char_while(content, char::is_whitespace);
        if size == 0 {
            return Err(ParseError::new(
                String::from("expected whitespace"),
                offset,
                offset + 1,
            ));
        }

        Ok((
            Whitespace {
                start: offset,
                end: offset + size,
            },
            size,
        ))
    }

    fn range(&self) -> (usize, usize) {
        (self.start, self.end)
    }
}

impl GrammarUnit for BareWord {
    fn try_match(content: &str, offset: usize) -> Result<(Self, usize)> {
        let size = take_char_while(content, |c| char::is_alphanumeric(c) || c == '_');
        if size == 0 {
            return Err(ParseError::new(
                String::from("expected bare word"),
                offset,
                offset + 1,
            ));
        }

        Ok((
            BareWord {
                start: offset,
                end: offset + size,
            },
            size,
        ))
    }

    fn range(&self) -> (usize, usize) {
        (self.start, self.end)
    }
}

impl GrammarUnit for Numeric {
    fn try_match(content: &str, offset: usize) -> Result<(Self, usize)> {
        let size = take_char_while(content, |c| {
            char::is_numeric(c) || c == '+' || c == '-' || c == '.' || c == 'e' || c == 'E'
        });
        if size == 0 {
            return Err(ParseError::new(
                String::from("expected number"),
                offset,
                offset + 1,
            ));
        }

        let value = match content[..size].parse::<f64>() {
            Ok(val) => val,
            Err(_) => {
                return Err(ParseError::new(
                    String::from("unable to parse number"),
                    offset,
                    offset + size,
                ));
            }
        };

        Ok((
            Numeric {
                start: offset,
                end: offset + size,
                value,
            },
            size,
        ))
    }

    fn range(&self) -> (usize, usize) {
        (self.start, self.end)
    }
}

impl GrammarUnit for Integer {
    fn try_match(content: &str, offset: usize) -> Result<(Self, usize)> {
        let size = take_char_while(content, |c| char::is_numeric(c) || c == '-' || c == '+');
        if size == 0 {
            return Err(ParseError::new(
                String::from("expected integer"),
                offset,
                offset + 1,
            ));
        }

        let value = match content[..size].parse::<i64>() {
            Ok(val) => val,
            Err(_) => {
                return Err(ParseError::new(
                    String::from("unable to parse integer"),
                    offset,
                    offset + size,
                ));
            }
        };

        Ok((
            Integer {
                start: offset,
                end: offset + size,
                value,
            },
            size,
        ))
    }

    fn range(&self) -> (usize, usize) {
        (self.start, self.end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_range<G: GrammarUnit>(content: &str, expected: &str) {
        let result = match G::try_match(content, 0) {
            Ok((g, _)) => g,
            Err(_) => {
                if expected.is_empty() {
                    return;
                } else {
                    panic!("{} didn't match!", std::any::type_name::<G>());
                }
            }
        };
        let (start, end) = result.range();
        assert_eq!(
            expected,
            format!("{}{}", " ".repeat(start), "^".repeat(end - start),)
        );
    }

    #[test]
    fn test_quoted_string_match() {
        let (qs, took) = QuotedString::try_match(r#""hello, world" test"#, 0).unwrap();
        assert_eq!(took, 14);
        assert_eq!(&qs.value, "hello, world");

        assert!(QuotedString::try_match("", 0).is_err());
        let (qs, took) = QuotedString::try_match(r#""my ' string \" test""#, 0).unwrap();
        assert_eq!(took, 21);
        assert_eq!(&qs.value, "my ' string \" test");

        assert_range::<QuotedString>(
            r#""hello, world" test"#,
            r#"^^^^^^^^^^^^^^"#, // comment to prevent reformat
        )
    }

    #[test]
    fn test_whitespace_match() {
        let content = r#""hello, world"   test"#;
        let (_, took) = QuotedString::try_match(content, 0).unwrap();
        assert_eq!(took, 14);

        let remaining = &content[took..];

        let (_, took) = Whitespace::try_match(remaining, took).unwrap();

        assert_eq!(took, 3);

        assert_range::<Whitespace>(
            "     test",
            "^^^^^", // comment to prevent reformat
        )
    }
}
