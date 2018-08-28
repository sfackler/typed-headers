use http::header::{self, HeaderName, HeaderValue, ACCESS_CONTROL_ALLOW_ORIGIN};
use {util, Error, Header, ToValues, ErrorKind};

/// The `Access-Control-Allow-Origin` response header,
/// part of [CORS](http://www.w3.org/TR/cors/#access-control-allow-origin-response-header)
///
/// The `Access-Control-Allow-Origin` header indicates whether a resource
/// can be shared based by returning the value of the Origin request header,
/// `*`, or `null` in the response.
///
/// # ABNF
///
/// ```text
/// Access-Control-Allow-Origin = "Access-Control-Allow-Origin" ":" origin-list-or-null | "*"
/// ```
///
/// # Example values
/// * `null`
/// * `*`
/// * `http://google.com/`
#[derive(Clone, PartialEq, Debug)]
pub enum AccessControlAllowOrigin {
    /// Allow all origins
    Any,
    /// A hidden origin
    Null,
    /// Allow one particular origin
    Value(String),
}

impl Header for AccessControlAllowOrigin {
    fn name() -> &'static HeaderName {
        &ACCESS_CONTROL_ALLOW_ORIGIN
    }

    #[inline]
    fn from_values(
        values: &mut header::ValueIter<HeaderValue>,
    ) -> Result<Option<AccessControlAllowOrigin>, Error> {
        if let Some(line) = values.next() {
            Ok(Some(match line.to_str().map_err(|_| Error(ErrorKind::InvalidValue))? {
                "*" => AccessControlAllowOrigin::Any,
                "null" => AccessControlAllowOrigin::Null,
                url => AccessControlAllowOrigin::Value(url.into())
            }))
        } else {
            Ok(None)
        }
    }

    #[inline]
    fn to_values(&self, values: &mut ToValues) {
        match *self {
            AccessControlAllowOrigin::Any => util::encode_single_value("*", values),
            AccessControlAllowOrigin::Null => util::encode_single_value("null", values),
            AccessControlAllowOrigin::Value(ref url) => util::encode_single_value(url, values),
        }
    }
}

