use base64;
use std::fmt;
use std::str::FromStr;

use {AuthScheme, Error, Token68};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Info {
    None,
    Token68(Token68),
    // AuthParams(Vec<(String, String)>),
}

/// Authentication credentials, as described in [RFC7235].
///
/// [RFC7235]: https://tools.ietf.org/html/rfc7235#section-2.1
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Credentials {
    scheme: AuthScheme,
    info: Info,
}

impl Credentials {
    /// Creates credentials from just an auth-scheme.
    #[inline]
    pub fn from_auth_scheme(scheme: AuthScheme) -> Credentials {
        Credentials {
            scheme,
            info: Info::None,
        }
    }

    /// Creates credentials from an auth-scheme and token68 data.
    #[inline]
    pub fn from_token68(scheme: AuthScheme, token: Token68) -> Credentials {
        Credentials {
            scheme,
            info: Info::Token68(token),
        }
    }

    /// Creates Bearer authentication credentials as described in [RFC6750].
    ///
    /// [RFC6750]: https://tools.ietf.org/html/rfc6750
    #[inline]
    pub fn bearer(token: Token68) -> Credentials {
        Credentials::from_token68(AuthScheme::BEARER, token)
    }

    /// Creates Basic authentication credentials as described in [RFC7617].
    ///
    /// [RFC7671]: https://tools.ietf.org/html/rfc7617
    #[inline]
    pub fn basic(user_id: &str, password: &str) -> Result<Credentials, Error> {
        if user_id.contains(':') || has_ctr(user_id) {
            return Err(Error::invalid_value());
        }

        if has_ctr(password) {
            return Err(Error::invalid_value());
        }

        let token = format!("{}:{}", user_id, password);
        let token = base64::encode(token.as_bytes());
        let token = Token68(token);

        Ok(Credentials::from_token68(AuthScheme::BASIC, token))
    }

    /// Returns the auth-scheme associated with the credentials.
    #[inline]
    pub fn scheme(&self) -> &AuthScheme {
        &self.scheme
    }

    /// Returns the token68 value associated with the credentials if present.
    #[inline]
    pub fn token68(&self) -> Option<&Token68> {
        match self.info {
            Info::None => None,
            Info::Token68(ref token) => Some(token),
        }
    }

    /// Returns the bearer token if this contains Bearer credentials.
    #[inline]
    pub fn as_bearer(&self) -> Option<&Token68> {
        if self.scheme != AuthScheme::BEARER {
            return None;
        }

        self.token68()
    }
}

impl fmt::Display for Credentials {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self.info {
            Info::None => fmt.write_str(self.scheme.as_str()),
            Info::Token68(ref token) => write!(fmt, "{} {}", self.scheme, token),
        }
    }
}

impl FromStr for Credentials {
    type Err = Error;

    fn from_str(s: &str) -> Result<Credentials, Error> {
        let mut it = s.splitn(2, ' ');
        let auth_scheme = it
            .next()
            .unwrap()
            .parse::<AuthScheme>()
            .map_err(|_| Error::invalid_value())?;

        let info = match it.next() {
            Some(info) => info,
            None => return Ok(Credentials::from_auth_scheme(auth_scheme)),
        };

        let info = info.trim_left_matches(' ');

        match info.parse::<Token68>() {
            Ok(token) => Ok(Credentials::from_token68(auth_scheme, token)),
            // FIXME parse out auth-params
            Err(_) => return Err(Error::invalid_value()),
        }
    }
}

fn has_ctr(s: &str) -> bool {
    s.as_bytes().iter().any(u8::is_ascii_control)
}
