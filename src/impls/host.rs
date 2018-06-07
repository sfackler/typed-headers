use bytes::Bytes;
use http::header::{self, HeaderName, HeaderValue, HOST};
use http::uri::Authority;

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
#[derive(Debug, Clone)]
pub struct Host {
    host: String,
    port: Option<u16>,
}

impl Host {
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
    fn parse<'a>(values: &mut header::ValueIter<'a, HeaderValue>) -> Result<Option<Host>, Error> {
        let value = match values.next() {
            Some(value) => value,
            None => return Ok(None),
        };

        let authority =
            Authority::from_shared(Bytes::from(value.as_bytes())).map_err(Error::custom)?;
        if authority.as_str().contains('@') {
            return Err(Error::custom("Host header may not contain userinfo"));
        }

        Ok(Some(Host::from_authority(&authority)))
    }

    #[inline]
    fn to_values(&self, values: &mut ToValues) -> Result<(), Error> {
        let value = match self.port {
            Some(port) => HeaderValue::from_str(&format!("{}:{}", self.host, port)),
            None => HeaderValue::from_str(&self.host),
        };
        let value = value.expect("should have already validated contents");

        values.append(value);
        Ok(())
    }
}
