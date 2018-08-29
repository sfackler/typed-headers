use std::str::FromStr;
use fmt;
use http::header::CACHE_CONTROL;
use Error;

header! {
/// `Cache-Control` header, defined in [RFC7234](https://tools.ietf.org/html/rfc7234#section-5.2)
///
/// The `Cache-Control` header field is used to specify directives for
/// caches along the request/response chain.  Such cache directives are
/// unidirectional in that the presence of a directive in a request does
/// not imply that the same directive is to be given in the response.
///
/// # ABNF
///
/// ```text
/// Cache-Control   = 1#cache-directive
/// cache-directive = token [ "=" ( token / quoted-string ) ]
/// ```
///
/// # Example values
///
/// * `no-cache`
/// * `private, community="UCI"`
/// * `max-age=30`
    (CacheControl, CACHE_CONTROL) => (CacheDirective)*
}

/// `CacheControl` contains a list of these directives.
#[derive(PartialEq, Clone, Debug)]
pub enum CacheDirective {
    /// "no-cache"
    NoCache,
    /// "no-store"
    NoStore,
    /// "no-transform"
    NoTransform,
    /// "only-if-cached"
    OnlyIfCached,

    // request directives
    /// "max-age=delta"
    MaxAge(u32),
    /// "max-stale=delta"
    MaxStale(u32),
    /// "min-fresh=delta"
    MinFresh(u32),

    // response directives
    /// "must-revalidate"
    MustRevalidate,
    /// "public"
    Public,
    /// "private"
    Private,
    /// "proxy-revalidate"
    ProxyRevalidate,
    /// "s-maxage=delta"
    SMaxAge(u32),

    /// Extension directives. Optionally include an argument.
    Extension(String, Option<String>)
}

impl fmt::Display for CacheDirective {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::CacheDirective::*;
        fmt::Display::fmt(match *self {
            NoCache => "no-cache",
            NoStore => "no-store",
            NoTransform => "no-transform",
            OnlyIfCached => "only-if-cached",

            MaxAge(secs) => return write!(f, "max-age={}", secs),
            MaxStale(secs) => return write!(f, "max-stale={}", secs),
            MinFresh(secs) => return write!(f, "min-fresh={}", secs),

            MustRevalidate => "must-revalidate",
            Public => "public",
            Private => "private",
            ProxyRevalidate => "proxy-revalidate",
            SMaxAge(secs) => return write!(f, "s-maxage={}", secs),

            Extension(ref name, None) => &name[..],
            Extension(ref name, Some(ref arg)) => return write!(f, "{}={}", name, arg),

        }, f)
    }
}

impl FromStr for CacheDirective {
    type Err = Error;
    fn from_str(s: &str) -> Result<CacheDirective, Error> {
        use self::CacheDirective::*;
        match s {
            "no-cache" => Ok(NoCache),
            "no-store" => Ok(NoStore),
            "no-transform" => Ok(NoTransform),
            "only-if-cached" => Ok(OnlyIfCached),
            "must-revalidate" => Ok(MustRevalidate),
            "public" => Ok(Public),
            "private" => Ok(Private),
            "proxy-revalidate" => Ok(ProxyRevalidate),
            "" => Err(Error::invalid_value()),
            _ => match s.find('=') {
                Some(idx) if idx+1 < s.len() => match (&s[..idx], (&s[idx+1..]).trim_matches('"')) {
                    ("max-age" , secs) => secs.parse().map(MaxAge).map_err(|_| Error::invalid_value()),
                    ("max-stale", secs) => secs.parse().map(MaxStale).map_err(|_| Error::invalid_value()),
                    ("min-fresh", secs) => secs.parse().map(MinFresh).map_err(|_| Error::invalid_value()),
                    ("s-maxage", secs) => secs.parse().map(SMaxAge).map_err(|_| Error::invalid_value()),
                    (left, right) => Ok(Extension(left.to_owned(), Some(right.to_owned())))
                },
                Some(_) => Err(Error::invalid_value()),
                None => Ok(Extension(s.to_owned(), None))
            }
        }
    }
}
