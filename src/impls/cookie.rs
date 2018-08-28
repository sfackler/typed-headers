use std::borrow::Cow;
use std::fmt::{self, Write};
use http::header::{self, HeaderName, HeaderValue, COOKIE};
use {util, Error, Header, ToValues, ErrorKind};

use internals::VecMap;

/// `Cookie` header, defined in [RFC6265](http://tools.ietf.org/html/rfc6265#section-5.4)
///
/// If the user agent does attach a Cookie header field to an HTTP
/// request, the user agent must send the cookie-string
/// as the value of the header field.
///
/// When the user agent generates an HTTP request, the user agent MUST NOT
/// attach more than one Cookie header field.
///
/// # Example values
/// * `SID=31d4d96e407aad42`
/// * `SID=31d4d96e407aad42; lang=en-US`
#[derive(Clone)]
pub struct Cookie(VecMap<Cow<'static, str>, Cow<'static, str>>);

impl Cookie {
    /// Creates a new `Cookie` header.
    pub fn new() -> Cookie {
        Cookie(VecMap::with_capacity(0))
    }

    /// Sets a name and value for the `Cookie`.
    ///
    /// # Note
    ///
    /// This will remove all other instances with the same name,
    /// and insert the new value.
    pub fn set<K, V>(&mut self, key: K, value: V)
        where K: Into<Cow<'static, str>>,
              V: Into<Cow<'static, str>>
    {
        let key = key.into();
        let value = value.into();
        self.0.remove_all(&key);
        self.0.append(key, value);
    }

    /// Append a name and value for the `Cookie`.
    ///
    /// # Note
    ///
    /// Cookies are allowed to set a name with a
    /// a value multiple times. For example:
    pub fn append<K, V>(&mut self, key: K, value: V)
        where K: Into<Cow<'static, str>>,
              V: Into<Cow<'static, str>>
    {
        self.0.append(key.into(), value.into());
    }

    /// Get a value for the name, if it exists.
    ///
    /// # Note
    ///
    /// Only returns the first instance found. To access
    /// any other values associated with the name, parse
    /// the `str` representation.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).map(AsRef::as_ref)
    }

    /// Iterate cookies.
    ///
    /// Iterate cookie (key, value) in insertion order.
    pub fn iter(&self) -> CookieIter {
        CookieIter(self.0.iter())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl Header for Cookie {
    fn name() -> &'static HeaderName {
        &COOKIE
    }

    #[inline]
    fn from_values(
        values: &mut header::ValueIter<HeaderValue>,
    ) -> Result<Option<Cookie>, Error> {
        let mut vec_map = VecMap::<Cow<'static, str>, Cow<'static, str>>::with_capacity(0);
        for cookies_raw in values {
            let cookies_str = cookies_raw.to_str().map_err(|_| Error(ErrorKind::InvalidValue))?;
            for cookie_str in cookies_str.split(';') {
                let mut key_val = cookie_str.splitn(2, '=');
                let key_val = (key_val.next(), key_val.next());
                if let (Some(key), Some(val)) = key_val {
                    vec_map.insert(key.trim().to_owned().into(), val.trim().to_owned().into());
                }
            }
        }

        if vec_map.len() != 0 {
            Ok(Some(Cookie(vec_map)))
        } else {
            Ok(None)
        }
    }

    #[inline]
    fn to_values(&self, values: &mut ToValues) {
        let mut out = String::new();
        let mut iter = self.0.iter();
        if let Some(&(ref key, ref val)) = iter.next() {
            write!(out, "{}={}", key, val).unwrap();
        }
        for &(ref key, ref val) in iter {
            write!(out, "; {}={}", key, val).unwrap();
        }
        util::encode_single_value(&out, values);
    }
}

impl PartialEq for Cookie {
    fn eq(&self, other: &Cookie) -> bool {
        if self.0.len() == other.0.len() {
            for &(ref k, ref v) in self.0.iter() {
                if other.get(k) != Some(v) {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }
}

impl fmt::Debug for Cookie {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map()
            .entries(self.0.iter().map(|&(ref k, ref v)| (k, v)))
            .finish()
    }
}

/// Iterator for cookie.
#[derive(Debug)]
pub struct CookieIter<'a>(::std::slice::Iter<'a, (Cow<'static, str>, Cow<'static, str>)>);

impl<'a> Iterator for CookieIter<'a> {
    type Item = (&'a str, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|kv| (kv.0.as_ref(), kv.1.as_ref()))
    }
}

