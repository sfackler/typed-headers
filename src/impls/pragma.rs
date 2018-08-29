use std::fmt;
use http::header::{self, HeaderName, HeaderValue, PRAGMA};

use {Error, Header, ToValues, util};

/// The `Pragma` header defined by HTTP/1.0.
///
/// > The "Pragma" header field allows backwards compatibility with
/// > HTTP/1.0 caches, so that clients can specify a "no-cache" request
/// > that they will understand (as Cache-Control was not defined until
/// > HTTP/1.1).  When the Cache-Control header field is also present and
/// > understood in a request, Pragma is ignored.
/// > In HTTP/1.0, Pragma was defined as an extensible field for
/// > implementation-specified directives for recipients.  This
/// > specification deprecates such extensions to improve interoperability.
///
/// Spec: [https://tools.ietf.org/html/rfc7234#section-5.4][url]
///
/// [url]: https://tools.ietf.org/html/rfc7234#section-5.4
#[derive(Clone, PartialEq, Debug)]
pub enum Pragma {
    /// Corresponds to the `no-cache` value.
    NoCache,
    /// Every value other than `no-cache`.
    Ext(String),
}

impl Header for Pragma {
    fn name() -> &'static HeaderName {
        &PRAGMA
    }

    #[inline]
    fn from_values(
        values: &mut header::ValueIter<HeaderValue>,
    ) -> Result<Option<Pragma>, Error> {
        if let Some(value) = values.next() {
            let s = value.to_str().map_err(|_| Error::invalid_value())?;
            let slice = &s.to_ascii_lowercase()[..];
            match slice {
                "no-cache" => return Ok(Some(Pragma::NoCache)),
                _ => return Ok(Some(Pragma::Ext(s.into()))),
            }
        }
        Ok(None)
    }

    #[inline]
    fn to_values(&self, values: &mut ToValues) {
        util::encode_single_value(&self.to_string(), values);
    }
}

impl fmt::Display for Pragma {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            Pragma::NoCache => "no-cache",
            Pragma::Ext(ref string) => &string[..],
        })
    }
}
