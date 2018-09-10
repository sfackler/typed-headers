use std::fmt;
use std::str::{self, FromStr};
use header::{self, HeaderName, HeaderValue, STRICT_TRANSPORT_SECURITY};

use unicase;

use {util, Error, Header, ToValues};

/// `StrictTransportSecurity` header, defined in [RFC6797](https://tools.ietf.org/html/rfc6797)
///
/// This specification defines a mechanism enabling web sites to declare
/// themselves accessible only via secure connections and/or for users to be
/// able to direct their user agent(s) to interact with given sites only over
/// secure connections.  This overall policy is referred to as HTTP Strict
/// Transport Security (HSTS).  The policy is declared by web sites via the
/// Strict-Transport-Security HTTP response header field and/or by other means,
/// such as user agent configuration, for example.
///
/// # ABNF
///
/// ```text
///      [ directive ]  *( ";" [ directive ] )
///
///      directive                 = directive-name [ "=" directive-value ]
///      directive-name            = token
///      directive-value           = token | quoted-string
///
/// ```
///
/// # Example values
///
/// * `max-age=31536000`
/// * `max-age=15768000 ; includeSubDomains`
/// ```
#[derive(Clone, PartialEq, Debug)]
pub struct StrictTransportSecurity {
    /// Signals the UA that the HSTS Policy applies to this HSTS Host as well as
    /// any subdomains of the host's domain name.
    pub include_subdomains: bool,

    /// Specifies the number of seconds, after the reception of the STS header
    /// field, during which the UA regards the host (from whom the message was
    /// received) as a Known HSTS Host.
    pub max_age: u64
}

impl StrictTransportSecurity {
    /// Create an STS header that includes subdomains
    pub fn including_subdomains(max_age: u64) -> StrictTransportSecurity {
        StrictTransportSecurity {
            max_age: max_age,
            include_subdomains: true
        }
    }

    /// Create an STS header that excludes subdomains
    pub fn excluding_subdomains(max_age: u64) -> StrictTransportSecurity {
        StrictTransportSecurity {
            max_age: max_age,
            include_subdomains: false
        }
    }
}

enum Directive {
    MaxAge(u64),
    IncludeSubdomains,
    Unknown
}

impl FromStr for StrictTransportSecurity {
    type Err = Error;

    fn from_str(s: &str) -> Result<StrictTransportSecurity, Error> {
        s.split(';')
            .map(str::trim)
            .map(|sub| if unicase::eq_ascii(sub, "includeSubdomains") {
                Ok(Directive::IncludeSubdomains)
            } else {
                let mut sub = sub.splitn(2, '=');
                match (sub.next(), sub.next()) {
                    (Some(left), Some(right))
                    if unicase::eq_ascii(left.trim(), "max-age") => {
                        right
                            .trim()
                            .trim_matches('"')
                            .parse()
                            .map(Directive::MaxAge)
                    },
                    _ => Ok(Directive::Unknown)
                }
            })
            .fold(Ok((None, None)), |res, dir| match (res, dir) {
                (Ok((None, sub)), Ok(Directive::MaxAge(age))) => Ok((Some(age), sub)),
                (Ok((age, None)), Ok(Directive::IncludeSubdomains)) => Ok((age, Some(()))),
                (Ok((Some(_), _)), Ok(Directive::MaxAge(_))) |
                (Ok((_, Some(_))), Ok(Directive::IncludeSubdomains)) |
                (_, Err(_)) => Err(Error::invalid_value()),
                (res, _) => res
            })
            .and_then(|res| match res {
                (Some(age), sub) => Ok(StrictTransportSecurity {
                    max_age: age,
                    include_subdomains: sub.is_some()
                }),
                _ => Err(Error::invalid_value())
            })
    }
}

impl Header for StrictTransportSecurity {
    fn name() -> &'static HeaderName {
        &STRICT_TRANSPORT_SECURITY
    }

    #[inline]
    fn from_values(
        values: &mut header::ValueIter<HeaderValue>,
    ) -> Result<Option<Self>, Error> {
        util::parse_single_value(values)
    }

    #[inline]
    fn to_values(&self, values: &mut ToValues) {
        util::encode_single_value(&self.to_string(), values);
    }
}

impl fmt::Display for StrictTransportSecurity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.include_subdomains {
            write!(f, "max-age={}; includeSubdomains", self.max_age)
        } else {
            write!(f, "max-age={}", self.max_age)
        }
    }
}

