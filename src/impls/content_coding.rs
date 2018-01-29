use std::fmt;
use std::error;
use std::str::FromStr;

use util;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Inner {
    Brotli,
    Gzip,
    Deflate,
    Compress,
    Identity,
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContentCoding(Inner);

impl ContentCoding {
    pub const BROTLI: ContentCoding = ContentCoding(Inner::Brotli);

    pub const GZIP: ContentCoding = ContentCoding(Inner::Gzip);

    pub const DEFLATE: ContentCoding = ContentCoding(Inner::Deflate);

    pub const COMPRESS: ContentCoding = ContentCoding(Inner::Compress);

    pub const IDENTITY: ContentCoding = ContentCoding(Inner::Identity);

    pub fn new(encoding: &str) -> Result<ContentCoding, InvalidContentCoding> {
        if encoding.eq_ignore_ascii_case("br") {
            Ok(ContentCoding::BROTLI)
        } else if encoding.eq_ignore_ascii_case("gzip") {
            Ok(ContentCoding::GZIP)
        } else if encoding.eq_ignore_ascii_case("x-gzip") {
            Ok(ContentCoding::GZIP)
        } else if encoding.eq_ignore_ascii_case("deflate") {
            Ok(ContentCoding::DEFLATE)
        } else if encoding.eq_ignore_ascii_case("compress") {
            Ok(ContentCoding::COMPRESS)
        } else if encoding.eq_ignore_ascii_case("x-compress") {
            Ok(ContentCoding::COMPRESS)
        } else if encoding.eq_ignore_ascii_case("identity") {
            Ok(ContentCoding::IDENTITY)
        } else {
            if util::is_token(encoding) {
                Ok(ContentCoding(Inner::Other(encoding.to_ascii_lowercase())))
            } else {
                Err(InvalidContentCoding(()))
            }
        }
    }

    pub fn as_str(&self) -> &str {
        match self.0 {
            Inner::Brotli => "br",
            Inner::Gzip => "gzip",
            Inner::Deflate => "deflate",
            Inner::Compress => "compress",
            Inner::Identity => "identity",
            Inner::Other(ref s) => s,
        }
    }
}

impl fmt::Display for ContentCoding {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(self.as_str())
    }
}

impl FromStr for ContentCoding {
    type Err = InvalidContentCoding;

    #[inline]
    fn from_str(s: &str) -> Result<ContentCoding, InvalidContentCoding> {
        ContentCoding::new(s)
    }
}

#[derive(Debug)]
pub struct InvalidContentCoding(());

impl fmt::Display for InvalidContentCoding {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("invalid content-coding")
    }
}

impl error::Error for InvalidContentCoding {
    fn description(&self) -> &str {
        "invalid content-coding"
    }
}
