use bytes::Bytes;
use http::header::{self, HeaderName, HeaderValue, HOST};
use http::uri::Authority;
use std::fmt;

use {Error, Header, ToValues};

/// The `Host` header, defined in [RFC7230].
///
/// The "Host" header field in a request provides the host and port
/// information from the target URI, enabling the origin server to
/// distinguish among resources while servicing requests for multiple
/// host names on a single IP address.
///
/// # ABNF
///
/// ```text
/// Host = uri-host [ ":" port ]
/// ```
///
/// [RFC7230]: https://tools.ietf.org/html/rfc7230#section-5.4
#[derive(Debug, Clone, PartialEq)]
pub struct Host {
    host: String,
    port: Option<u16>,
}

impl Host {
    /// Creates a Host header from a hostname and optional port.
    #[inline]
    pub fn new(host: &str, port: Option<u16>) -> Result<Host, Error> {
        // go through authority to validate the hostname
        let authority = match port {
            Some(port) => Bytes::from(format!("{}:{}", host, port)),
            None => Bytes::from(host),
        };
        let authority = Authority::from_shared(authority).map_err(|_| Error::invalid_value())?;

        Ok(Host::from_authority(&authority))
    }

    /// Creates a Host header from a URI authority component.
    ///
    /// The userinfo portion of the authority is not included in the header.
    #[inline]
    pub fn from_authority(authority: &Authority) -> Host {
        Host {
            host: authority.host().to_string(),
            port: authority.port(),
        }
    }

    /// Returns the host.
    #[inline]
    pub fn host(&self) -> &str {
        &self.host
    }

    /// Returns the port.
    #[inline]
    pub fn port(&self) -> Option<u16> {
        self.port
    }
}

impl Header for Host {
    #[inline]
    fn name() -> &'static HeaderName {
        &HOST
    }

    #[inline]
    fn from_values<'a>(
        values: &mut header::ValueIter<'a, HeaderValue>,
    ) -> Result<Option<Host>, Error> {
        let value = match values.next() {
            Some(value) => value,
            None => return Ok(None),
        };

        let authority = Authority::from_shared(Bytes::from(value.as_bytes()))
            .map_err(|_| Error::invalid_value())?;
        // host header can't contain userinfo
        if authority.as_str().contains('@') {
            return Err(Error::invalid_value());
        }

        Ok(Some(Host::from_authority(&authority)))
    }

    #[inline]
    fn to_values(&self, values: &mut ToValues) {
        values.append(HeaderValue::from_str(&self.to_string()).unwrap());
    }
}

impl fmt::Display for Host {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.port {
            Some(port) => write!(f, "{}:{}", self.host, port),
            None => f.write_str(&self.host),
        }
    }
}
