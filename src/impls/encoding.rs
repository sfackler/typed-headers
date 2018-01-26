use std::fmt;
use std::str::FromStr;
use std::error;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Inner {
    Chunked,
    Brotli,
    Gzip,
    Deflate,
    Compress,
    Identity,
    Trailers,
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Encoding(Inner);

impl Encoding {
    pub const CHUNKED: Encoding = Encoding(Inner::Chunked);

    pub const BROTLI: Encoding = Encoding(Inner::Brotli);

    pub const GZIP: Encoding = Encoding(Inner::Gzip);

    pub const DEFLATE: Encoding = Encoding(Inner::Deflate);

    pub const COMPRESS: Encoding = Encoding(Inner::Compress);

    pub const IDENTITY: Encoding = Encoding(Inner::Identity);

    pub const TRAILERS: Encoding = Encoding(Inner::Trailers);

    pub fn new(encoding: &str) -> Encoding {
        match encoding {
            "chunked" => Encoding::CHUNKED,
            "br" => Encoding::BROTLI,
            "gzip" => Encoding::GZIP,
            "deflate" => Encoding::DEFLATE,
            "compress" => Encoding::COMPRESS,
            "identity" => Encoding::IDENTITY,
            "trailers" => Encoding::TRAILERS,
            s => Encoding(Inner::Other(s.to_string())),
        }
    }

    pub fn as_str(&self) -> &str {
        match self.0 {
            Inner::Chunked => "chunked",
            Inner::Brotli => "br",
            Inner::Gzip => "gzip",
            Inner::Deflate => "deflate",
            Inner::Compress => "compress",
            Inner::Identity => "identity",
            Inner::Trailers => "trailers",
            Inner::Other(ref s) => s,
        }
    }
}

impl fmt::Display for Encoding {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(self.as_str())
    }
}

impl FromStr for Encoding {
    type Err = Void;

    #[inline]
    fn from_str(s: &str) -> Result<Encoding, Void> {
        Ok(Encoding::new(s))
    }
}

#[doc(hidden)]
pub enum Void {}

impl From<Void> for Box<error::Error + Sync + Send> {
    fn from(v: Void) -> Box<error::Error + Sync + Send> {
        match v {}
    }
}
