use {util, Error, Header, ToValues, Host};
use bytes::Bytes;
use http::uri::Authority;
use http::header::{self, HeaderName, HeaderValue, ORIGIN};
use std::borrow::Cow;
use std::fmt;
use std::str::FromStr;

/// The `Origin` header.
///
/// The `Origin` header is a version of the `Referer` header that is used for all HTTP fetches and `POST`s whose CORS flag is set.
/// This header is often used to inform recipients of the security context of where the request was initiated.
///
/// Following the spec, [https://fetch.spec.whatwg.org/#origin-header][url], the value of this header is composed of
/// a String (scheme), Host (host/port)
///
/// [url]: https://fetch.spec.whatwg.org/#origin-header
#[derive(PartialEq, Clone, Debug)]
pub struct Origin(OriginOrNull);

#[derive(PartialEq, Clone, Debug)]
enum OriginOrNull {
    Origin {
        /// The scheme, such as http or https
        scheme: Cow<'static,str>,
        /// The host, such as Host{hostname: "hyper.rs".to_owned(), port: None}
        host: Host,
    },
    Null,
}

impl Origin {
    /// Creates a new `Origin` header.
    pub fn new<S: Into<Cow<'static,str>>, H: Into<Cow<'static,str>>>(scheme: S, hostname: H, port: Option<u16>) -> Result<Origin, Error> {
        Ok(Origin(OriginOrNull::Origin {
            scheme: scheme.into(),
            host: Host::new(&hostname.into(), port)?,
        }))
    }

    /// Creates a `Null` `Origin` header.
    pub fn null() -> Origin {
        Origin(OriginOrNull::Null)
    }

    /// Checks if `Origin` is `Null`.
    pub fn is_null(&self) -> bool {
        match self {
            &Origin(OriginOrNull::Null) => true,
            _ => false,
        }
    }

    /// The scheme, such as http or https.
    pub fn scheme(&self) -> Option<&str> {
        match self {
            &Origin(OriginOrNull::Origin { ref scheme, .. }) => Some(&scheme),
            _ => None,
        }
    }

    /// The host, such as `Host { hostname: "hyper.rs".to_owned(), port: None}`.
    pub fn host(&self) -> Option<&Host> {
        match self {
            &Origin(OriginOrNull::Origin { ref host, .. }) => Some(&host),
            _ => None,
        }
    }
}

impl Header for Origin {
    fn name() -> &'static HeaderName {
        &ORIGIN
    }

    fn from_values(
        values: &mut header::ValueIter<HeaderValue>,
    ) -> Result<Option<Origin>, Error> {
        if let Some(line) = values.next() {
            return line.to_str().map_err(|_| Error::invalid_value()).map(|v| v.parse().ok())
        }
        Ok(None)
    }

    #[inline]
    fn to_values(&self, values: &mut ToValues) {
        util::encode_single_value(&self.to_string(), values);
    }
}

static HTTP : &'static str = "http";
static HTTPS : &'static str = "https";

impl FromStr for Origin {
    type Err = Error;

    fn from_str(s: &str) -> Result<Origin, Error> {
        let idx = match s.find("://") {
            Some(idx) => idx,
            None => return Err(Error::invalid_value())
        };
        // idx + 3 because that's how long "://" is
        let (scheme, etc) = (&s[..idx], &s[idx + 3..]);
        let authority = Authority::from_shared(Bytes::from(etc.as_bytes()))
            .map_err(|_| Error::invalid_value())?;
        let host = Host::from_authority(&authority);
        let scheme = match scheme {
            "http"  => Cow::Borrowed(HTTP),
            "https" => Cow::Borrowed(HTTPS),
            s       => Cow::Owned(s.to_owned())
        };

        Ok(Origin(OriginOrNull::Origin {
            scheme: scheme,
            host: host
        }))
    }
}

impl fmt::Display for Origin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            OriginOrNull::Origin { ref scheme, ref host } => write!(f, "{}://{}", scheme, host),
            // Serialized as "null" per ASCII serialization of an origin
            // https://html.spec.whatwg.org/multipage/browsers.html#ascii-serialisation-of-an-origin
            OriginOrNull::Null => f.write_str("null")
        }
    }
}

