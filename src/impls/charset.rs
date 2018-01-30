use std::error;
use std::fmt;
use std::str::FromStr;

use util;

macro_rules! charset {
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
        pub struct Charset(Inner);

        impl PartialEq for Charset {
            fn eq(&self, other: &Charset) -> bool {
                match (&self.0, &other.0) {
                    $(
                        (&Inner::$variant, &Inner::$variant) => true,
                    )*
                    (&Inner::Other(ref a), &Inner::Other(ref b)) => a.eq_ignore_ascii_case(b),
                    _ => false,
                }
            }
        }

        impl Eq for Charset {}

        impl Charset {
            $(
                pub const $variant: Charset = Charset(Inner::$variant);
            )*

            pub fn new(charset: &str) -> Result<Charset, InvalidCharset> {
                $(
                    if charset.eq_ignore_ascii_case($str) {
                        return Ok(Charset(Inner::$variant));
                    }
                )*

                if util::is_token(charset) {
                    Ok(Charset(Inner::Other(charset.to_string())))
                } else {
                    Err(InvalidCharset(()))
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

        impl fmt::Display for Charset {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                fmt.write_str(self.as_str())
            }
        }

        impl FromStr for Charset {
            type Err = InvalidCharset;

            fn from_str(s: &str) -> Result<Charset, InvalidCharset> {
                Charset::new(s)
            }
        }
    }
}

charset! {
    (STAR, "*"),
    (US_ASCII, "us-ascii"),
    (ISO_8859_1, "iso-8859-1"),
    (ISO_8859_2, "iso-8859-2"),
    (ISO_8859_3, "iso-8859-3"),
    (ISO_8859_4, "iso-8859-4"),
    (ISO_8859_5, "iso-8859-5"),
    (ISO_8859_6, "iso-8859-6"),
    (ISO_8859_7, "iso-8859-7"),
    (ISO_8859_8, "iso-8859-8"),
    (ISO_8859_9, "iso-8859-9"),
    (ISO_8859_10, "iso-8859-10"),
    (SHIFT_JIS, "shift-jis"),
    (EUC_JP, "euc-jp"),
    (ISO_2022_KR, "iso-2022-kr"),
    (EUC_KR, "euc-kr"),
    (ISO_2022_JP, "iso-2022-jp"),
    (ISO_2022_JP_2, "iso-2022-jp-2"),
    (ISO_8859_6_E, "iso-8859-6-e"),
    (ISO_8859_6_I, "iso-8859-6-i"),
    (ISO_8859_8_E, "iso-8859-8-e"),
    (ISO_8859_8_I, "iso-8859-8-i"),
    (UTF_8, "utf-8"),
    (UTF_16BE, "utf-16be"),
    (UTF_16LE, "utf-16le"),
    (UTF_16, "utf-16"),
    (UTF_32, "utf-32"),
    (UTF_32BE, "utf-32be"),
    (UTF_32LE, "utf-32le"),
    (GB2312, "gb2312"),
    (BIG5, "big5"),
    (KOI8_R, "koi8-r"),
}

#[derive(Debug)]
pub struct InvalidCharset(());

impl fmt::Display for InvalidCharset {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("invalid charset")
    }
}

impl error::Error for InvalidCharset {
    fn description(&self) -> &str {
        "invalid charset"
    }
}
