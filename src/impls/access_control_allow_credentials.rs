use http::header::{self, HeaderName, HeaderValue, ACCESS_CONTROL_ALLOW_CREDENTIALS};
use {util, Error, Header, ToValues, ErrorKind};

/// `Access-Control-Allow-Credentials` header, part of
/// [CORS](http://www.w3.org/TR/cors/#access-control-allow-headers-response-header)
///
/// > The Access-Control-Allow-Credentials HTTP response header indicates whether the
/// > response to request can be exposed when the credentials flag is true. When part
/// > of the response to an preflight request it indicates that the actual request can
/// > be made with credentials. The Access-Control-Allow-Credentials HTTP header must
/// > match the following ABNF:
///
/// # ABNF
///
/// ```text
/// Access-Control-Allow-Credentials: "Access-Control-Allow-Credentials" ":" "true"
/// ```
///
/// Since there is only one acceptable field value, the header struct does not accept
/// any values at all. Setting an empty `AccessControlAllowCredentials` header is
/// sufficient. See the examples below.
///
/// # Example values
/// * "true"
#[derive(Clone, PartialEq, Debug)]
pub struct AccessControlAllowCredentials;

impl Header for AccessControlAllowCredentials {
    fn name() -> &'static HeaderName {
        &ACCESS_CONTROL_ALLOW_CREDENTIALS
    }

    #[inline]
    fn from_values(
        values: &mut header::ValueIter<HeaderValue>,
    ) -> Result<Option<AccessControlAllowCredentials>, Error> {
        if let Some(value) = values.next() {
            let text = value.to_str().map_err(|_| Error(ErrorKind::InvalidValue))?.to_lowercase();
            if text == "true" {
                return Ok(Some(AccessControlAllowCredentials));
            }
        }
        Ok(None)
    }

    #[inline]
    fn to_values(&self, values: &mut ToValues) {
        util::encode_single_value("true", values);
    }
}
