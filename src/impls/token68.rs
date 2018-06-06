use std::error::Error;
use std::fmt;
use std::str::FromStr;

/// A base68 value as defined in [RFC7235].
///
/// [RFC7235]: https://tools.ietf.org/html/rfc7235#section-2.1
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token68(pub(crate) String);

impl Token68 {
    /// Constructs a new base68 value.
    #[inline]
    pub fn new(s: &str) -> Result<Token68, InvalidToken68> {
        let trimmed = s.trim_right_matches('=');

        if trimmed.is_empty() {
            return Err(InvalidToken68(()));
        }

        let ok = trimmed.as_bytes().iter().all(|b| match b {
            b'0'...b'9' | b'a'...b'z' | b'A'...b'Z' | b'-' | b'.' | b'_' | b'~' | b'+' | b'/' => {
                true
            }
            _ => false,
        });

        if ok {
            Ok(Token68(s.to_string()))
        } else {
            Err(InvalidToken68(()))
        }
    }

    /// Returns the string the value as a string.
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Token68 {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(self.as_str())
    }
}

impl FromStr for Token68 {
    type Err = InvalidToken68;

    #[inline]
    fn from_str(s: &str) -> Result<Token68, InvalidToken68> {
        Token68::new(s)
    }
}

#[derive(Debug)]
pub struct InvalidToken68(());

impl fmt::Display for InvalidToken68 {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("invalid base68")
    }
}

impl Error for InvalidToken68 {
    fn description(&self) -> &str {
        "invalid base68"
    }
}
