use http::header::{self, HeaderName, HeaderValue, SET_COOKIE};
use {util, Error, Header, ToValues};


/// `Set-Cookie` header, defined [RFC6265](http://tools.ietf.org/html/rfc6265#section-4.1)
///
/// The Set-Cookie HTTP response header is used to send cookies from the
/// server to the user agent.
///
/// Informally, the Set-Cookie response header contains the header name
/// "Set-Cookie" followed by a ":" and a cookie.  Each cookie begins with
/// a name-value-pair, followed by zero or more attribute-value pairs.
///
/// # ABNF
///
/// ```text
/// set-cookie-header = "Set-Cookie:" SP set-cookie-string
/// set-cookie-string = cookie-pair *( ";" SP cookie-av )
/// cookie-pair       = cookie-name "=" cookie-value
/// cookie-name       = token
/// cookie-value      = *cookie-octet / ( DQUOTE *cookie-octet DQUOTE )
/// cookie-octet      = %x21 / %x23-2B / %x2D-3A / %x3C-5B / %x5D-7E
///                       ; US-ASCII characters excluding CTLs,
///                       ; whitespace DQUOTE, comma, semicolon,
///                       ; and backslash
/// token             = <token, defined in [RFC2616], Section 2.2>
///
/// cookie-av         = expires-av / max-age-av / domain-av /
///                    path-av / secure-av / httponly-av /
///                     extension-av
/// expires-av        = "Expires=" sane-cookie-date
/// sane-cookie-date  = <rfc1123-date, defined in [RFC2616], Section 3.3.1>
/// max-age-av        = "Max-Age=" non-zero-digit *DIGIT
///                       ; In practice, both expires-av and max-age-av
///                       ; are limited to dates representable by the
///                       ; user agent.
/// non-zero-digit    = %x31-39
///                       ; digits 1 through 9
/// domain-av         = "Domain=" domain-value
/// domain-value      = <subdomain>
///                       ; defined in [RFC1034], Section 3.5, as
///                       ; enhanced by [RFC1123], Section 2.1
/// path-av           = "Path=" path-value
/// path-value        = <any CHAR except CTLs or ";">
/// secure-av         = "Secure"
/// httponly-av       = "HttpOnly"
/// extension-av      = <any CHAR except CTLs or ";">
/// ```
///
/// # Example values
///
/// * `SID=31d4d96e407aad42`
/// * `lang=en-US; Expires=Wed, 09 Jun 2021 10:18:14 GMT`
/// * `lang=; Expires=Sun, 06 Nov 1994 08:49:37 GMT`
/// * `lang=en-US; Path=/; Domain=example.com`
#[derive(Clone, PartialEq, Debug)]
pub struct SetCookie(pub Vec<String>);

//__hyper__deref!(SetCookie => Vec<String>);

impl Header for SetCookie {
    fn name() -> &'static HeaderName {
        &SET_COOKIE
    }

    fn from_values(
        values: &mut header::ValueIter<HeaderValue>,
    ) -> Result<Option<SetCookie>, Error> {
        let mut set_cookies = vec![];
        for set_cookies_raw in values {
            if let Ok(s) = set_cookies_raw.to_str() {
                set_cookies.push(s.trim().to_owned());
            }
        }

        if !set_cookies.is_empty() {
            Ok(Some(SetCookie(set_cookies)))
        } else {
            Err(Error::invalid_value())
        }
    }

    #[inline]
    fn to_values(&self, values: &mut ToValues) {
        for cookie in &self.0 {
            util::encode_single_value(cookie, values);
        }
    }
}
