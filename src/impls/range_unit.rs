use std::error;
use std::fmt;
use std::str::FromStr;

use util;

macro_rules! range_unit {
    ($(($variant:ident, $str:expr),)*) => {
        #[derive(Debug, Clone)]
        #[allow(non_camel_case_types)]
        enum Inner {
            $(
                $variant,
            )*
            Other(String),
        }

        #[derive(Debug, Clone)]
        pub struct RangeUnit(Inner);

        impl PartialEq for RangeUnit {
            fn eq(&self, other: &RangeUnit) -> bool {
                match (&self.0, &other.0) {
                    $(
                        (&Inner::$variant, &Inner::$variant) => true,
                    )*
                    (&Inner::Other(ref a), &Inner::Other(ref b)) => a.eq_ignore_ascii_case(b),
                    _ => false,
                }
            }
        }

        impl Eq for RangeUnit {}

        impl RangeUnit {
            $(
                pub const $variant: RangeUnit = RangeUnit(Inner::$variant);
            )*

            pub fn new(charset: &str) -> Result<RangeUnit, InvalidRangeUnit> {
                $(
                    if charset.eq_ignore_ascii_case($str) {
                        return Ok(RangeUnit(Inner::$variant));
                    }
                )*

                if util::is_token(charset) {
                    Ok(RangeUnit(Inner::Other(charset.to_string())))
                } else {
                    Err(InvalidRangeUnit(()))
                }
            }

            pub fn as_str(&self) -> &str {
                match self.0 {
                    $(
                        Inner::$variant => $str,
                    )*
                    Inner::Other(ref s) => s,
                }
            }
        }

        impl fmt::Display for RangeUnit {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                fmt.write_str(self.as_str())
            }
        }

        impl FromStr for RangeUnit {
            type Err = InvalidRangeUnit;

            fn from_str(s: &str) -> Result<RangeUnit, InvalidRangeUnit> {
                RangeUnit::new(s)
            }
        }
    }
}

range_unit! {
    (BYTES, "bytes"),
    (NONE, "none"),
}

#[derive(Debug)]
pub struct InvalidRangeUnit(());

impl fmt::Display for InvalidRangeUnit {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("invalid range-unit")
    }
}

impl error::Error for InvalidRangeUnit {
    fn description(&self) -> &str {
        "invalid range-unit"
    }
}
