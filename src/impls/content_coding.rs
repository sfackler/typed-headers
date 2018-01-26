use std::fmt;
use std::str::FromStr;
use std::error;

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

    pub fn new(encoding: &str) -> ContentCoding {
        if encoding.eq_ignore_ascii_case("br") {
            ContentCoding::BROTLI
        } else if encoding.eq_ignore_ascii_case("gzip") {
            ContentCoding::GZIP
        } else if encoding.eq_ignore_ascii_case("x-gzip") {
            ContentCoding::GZIP
        } else if encoding.eq_ignore_ascii_case("deflate") {
            ContentCoding::DEFLATE
        } else if encoding.eq_ignore_ascii_case("compress") {
            ContentCoding::COMPRESS
        } else if encoding.eq_ignore_ascii_case("x-compress") {
            ContentCoding::COMPRESS
        } else if encoding.eq_ignore_ascii_case("identity") {
            ContentCoding::IDENTITY
        } else {
            // FIXME check for invalid characters?
            ContentCoding(Inner::Other(encoding.to_ascii_lowercase()))
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
    type Err = Void;

    #[inline]
    fn from_str(s: &str) -> Result<ContentCoding, Void> {
        Ok(ContentCoding::new(s))
    }
}

#[doc(hidden)]
pub enum Void {}

impl From<Void> for Box<error::Error + Sync + Send> {
    fn from(v: Void) -> Box<error::Error + Sync + Send> {
        match v {}
    }
}
