use std::fmt;
use std::slice;
use std::str::{self, FromStr};

/// A value paired with its "quality" as defined in [RFC7231].
///
/// Quality items are used in content negotiation headers such as `Accept` and `Accept-Encoding`.
///
/// [RFC7231]: https://tools.ietf.org/html/rfc7231#section-5.3
#[derive(Debug, Clone, PartialEq)]
pub struct QualityItem<T> {
    pub item: T,
    pub quality: Quality,
}

impl<T> QualityItem<T> {
    /// Creates a new quality item.
    pub fn new(item: T, quality: Quality) -> QualityItem<T> {
        QualityItem { item, quality }
    }
}

impl<T> fmt::Display for QualityItem<T>
where
    T: fmt::Display,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.item, fmt)?;
        match self.quality.0 {
            1000 => Ok(()),
            0 => fmt.write_str("; q=0"),
            mut x => {
                fmt.write_str("; q=0.")?;
                let mut digits = *b"000";
                digits[2] = (x % 10) as u8 + b'0';
                x /= 10;
                digits[1] = (x % 10) as u8 + b'0';
                x /= 10;
                digits[0] = (x % 10) as u8 + b'0';

                let s = unsafe { str::from_utf8_unchecked(&digits[..]) };
                fmt.write_str(s.trim_right_matches('0'))
            }
        }
    }
}

impl<T> FromStr for QualityItem<T>
where
    T: FromStr,
{
    type Err = T::Err;

    fn from_str(mut s: &str) -> Result<QualityItem<T>, T::Err> {
        let quality = match WeightParser::parse(s) {
            Some((remaining, quality)) => {
                s = &s[..remaining];
                quality
            }
            None => Quality(1000),
        };

        let item = s.parse()?;

        Ok(QualityItem { item, quality })
    }
}

struct WeightParser<'a>(slice::Iter<'a, u8>);

impl<'a> WeightParser<'a> {
    fn parse(s: &'a str) -> Option<(usize, Quality)> {
        let mut parser = WeightParser(s.as_bytes().iter());
        let qvalue = parser.qvalue()?;
        parser.eat(b'=')?;
        parser.eat(b'q').or_else(|| parser.eat(b'Q'))?;
        parser.ows();
        parser.eat(b';')?;
        parser.ows();
        let remaining = parser.0.as_slice().len();
        Some((remaining, Quality(qvalue)))
    }

    fn qvalue(&mut self) -> Option<u16> {
        let mut qvalue = match self.digit() {
            Some(v @ 0) | Some(v @ 1) if self.peek() == Some(b'=') => return Some(v * 1000),
            Some(v) => v,
            None if self.peek() == Some(b'.') => 0,
            None => return None,
        };

        match self.digit() {
            Some(digit1) => match self.digit() {
                Some(digit2) => qvalue += digit1 * 10 + digit2 * 100,
                None => {
                    qvalue *= 10;
                    qvalue += digit1 * 100;
                }
            },
            None => qvalue *= 100,
        }

        self.eat(b'.')?;

        match self.peek()? {
            b'0' => {
                self.next();
                Some(qvalue)
            }
            b'1' if qvalue == 0 => {
                self.next();
                Some(1000)
            }
            _ => None,
        }
    }

    fn digit(&mut self) -> Option<u16> {
        match self.peek()? {
            v @ b'0'...b'9' => {
                self.next();
                Some((v - b'0') as u16)
            }
            _ => None,
        }
    }

    fn ows(&mut self) {
        loop {
            match self.peek() {
                Some(b' ') | Some(b'\t') => {
                    self.next();
                }
                _ => break,
            }
        }
    }

    fn peek(&self) -> Option<u8> {
        self.0.clone().next_back().cloned()
    }

    fn next(&mut self) -> Option<u8> {
        self.0.next_back().cloned()
    }

    fn eat(&mut self, value: u8) -> Option<()> {
        if self.peek() == Some(value) {
            self.next();
            Some(())
        } else {
            None
        }
    }
}

/// A quality value, as specified in [RFC7231].
///
/// Quality values are decimal numbers between 0 and 1 (inclusive) with up to 3 fractional digits of precision.
///
/// [RFC7231]: https://tools.ietf.org/html/rfc7231#section-5.3.1
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Quality(u16);

impl Quality {
    /// Creates a quality value from a value between 0 and 1000 inclusive.
    ///
    /// This is semantically divided by 1000 to produce a value between 0 and 1.
    ///
    /// # Panics
    ///
    /// Panics if the value is greater than 1000.
    pub fn from_u16(quality: u16) -> Quality {
        assert!(quality <= 1000);
        Quality(quality)
    }

    /// Returns the quality multiplied by 1000 as an integer.
    pub fn as_u16(&self) -> u16 {
        self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use Error;

    #[derive(Debug, Clone, PartialEq)]
    struct Item;

    impl fmt::Display for Item {
        fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            fmt.write_str("item")
        }
    }

    impl FromStr for Item {
        type Err = Error;

        fn from_str(s: &str) -> Result<Item, Error> {
            if s == "item" {
                Ok(Item)
            } else {
                Err(Error::invalid_value())
            }
        }
    }

    fn qitem(quality: u16) -> QualityItem<Item> {
        QualityItem {
            item: Item,
            quality: Quality(quality),
        }
    }

    #[test]
    fn parse_ok() {
        assert_eq!(qitem(1000), "item".parse().unwrap());
        assert_eq!(qitem(1000), "item; q=1".parse().unwrap());
        assert_eq!(qitem(1000), "item; Q=1".parse().unwrap());
        assert_eq!(qitem(1000), "item ;q=1".parse().unwrap());
        assert_eq!(qitem(1000), "item; q=1.".parse().unwrap());
        assert_eq!(qitem(1000), "item; q=1.0".parse().unwrap());
        assert_eq!(qitem(1000), "item; q=1.00".parse().unwrap());
        assert_eq!(qitem(1000), "item; q=1.000".parse().unwrap());

        assert_eq!(qitem(0), "item; q=0".parse().unwrap());
        assert_eq!(qitem(0), "item; q=0.".parse().unwrap());
        assert_eq!(qitem(0), "item; q=0.0".parse().unwrap());
        assert_eq!(qitem(0), "item; q=0.00".parse().unwrap());
        assert_eq!(qitem(0), "item; q=0.000".parse().unwrap());

        assert_eq!(qitem(100), "item; q=0.1".parse().unwrap());
        assert_eq!(qitem(100), "item; q=0.10".parse().unwrap());
        assert_eq!(qitem(100), "item; q=0.100".parse().unwrap());
        assert_eq!(qitem(120), "item; q=0.12".parse().unwrap());
        assert_eq!(qitem(120), "item; q=0.120".parse().unwrap());
        assert_eq!(qitem(123), "item; q=0.123".parse().unwrap());
    }

    #[test]
    fn parse_err() {
        assert!("item; q=".parse::<QualityItem<Item>>().is_err());
        assert!("item; q=.1".parse::<QualityItem<Item>>().is_err());
        assert!("item; q=1.1".parse::<QualityItem<Item>>().is_err());
        assert!("item; q=1.01".parse::<QualityItem<Item>>().is_err());
        assert!("item; q=1.001".parse::<QualityItem<Item>>().is_err());
        assert!("item; q=0.0001".parse::<QualityItem<Item>>().is_err());
    }

    #[test]
    fn display() {
        assert_eq!(qitem(1000).to_string(), "item");
        assert_eq!(qitem(0).to_string(), "item; q=0");
        assert_eq!(qitem(1).to_string(), "item; q=0.001");
        assert_eq!(qitem(10).to_string(), "item; q=0.01");
        assert_eq!(qitem(100).to_string(), "item; q=0.1");
    }
}
