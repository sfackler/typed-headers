pub use impls::accept::Accept;
pub use impls::accept_encoding::AcceptEncoding;
pub use impls::access_control_allow_credentials::AccessControlAllowCredentials;
pub use impls::access_control_allow_headers::AccessControlAllowHeaders;
pub use impls::access_control_allow_methods::AccessControlAllowMethods;
pub use impls::access_control_allow_origin::AccessControlAllowOrigin;
pub use impls::access_control_max_age::AccessControlMaxAge;
pub use impls::access_control_expose_headers::AccessControlExposeHeaders;
pub use impls::access_control_request_method::AccessControlRequestMethod;
pub use impls::access_control_request_headers::AccessControlRequestHeaders;
pub use impls::accept_language::AcceptLanguage;
pub use impls::allow::Allow;
pub use impls::auth_scheme::AuthScheme;
pub use impls::authorization::Authorization;
pub use impls::cache_control::{CacheControl, CacheDirective};
pub use impls::content_coding::ContentCoding;
pub use impls::content_disposition::{ContentDisposition, DispositionParam, DispositionType};
pub use impls::content_encoding::ContentEncoding;
pub use impls::content_length::ContentLength;
pub use impls::content_location::ContentLocation;
pub use impls::content_type::ContentType;
pub use impls::content_range::{ContentRange, ContentRangeSpec};
pub use impls::cookie::Cookie;
pub use impls::credentials::Credentials;
pub use impls::etag::ETag;
pub use impls::expires::Expires;
pub use impls::host::Host;
pub use impls::http_date::HttpDate;
pub use impls::if_none_match::IfNoneMatch;
pub use impls::if_modified_since::IfModifiedSince;
pub use impls::last_modified::LastModified;
pub use impls::location::Location;
pub use impls::origin::Origin;
pub use impls::pragma::Pragma;
pub use impls::proxy_authorization::ProxyAuthorization;
pub use impls::quality::{Quality, QualityItem};
pub use impls::range::{Range, ByteRangeSpec};
pub use impls::referer::Referer;
pub use impls::referrer_policy::ReferrerPolicy;
pub use impls::retry_after::RetryAfter;
pub use impls::set_cookie::SetCookie;
pub use impls::token68::Token68;
pub use impls::user_agent::UserAgent;
pub use impls::vary::Vary;

macro_rules! header {
    // #rule
    ($(#[$a:meta])*($id:ident, $n:expr) => ($item:ty)*) => {
        $(#[$a])*
        #[derive(Clone, Debug, PartialEq)]
        pub struct $id(pub ::std::vec::Vec<$item>);
        header!(@deref $id => Vec<$item>);
        header!(@derefmut $id => Vec<$item>);
        impl $crate::Header for $id {
            #[inline]
            fn name() -> &'static $crate::http::header::HeaderName {
                &$n
            }

            #[inline]
            fn from_values(
                values: &mut $crate::http::header::ValueIter<$crate::http::header::HeaderValue>,
            ) -> ::std::result::Result<::std::option::Option<$id>, $crate::Error>
            {
                $crate::util::parse_comma_delimited(values).map(|r| r.map($id))
            }

            #[inline]
            fn to_values(&self, values: &mut $crate::ToValues) {
                $crate::util::encode_comma_delimited(&self.0, values);
            }
        }
    };
    // 1#rule
    ($(#[$a:meta])*($id:ident, $n:expr) => ($item:ty)+) => {
        $(#[$a])*
        #[derive(Clone, Debug, PartialEq)]
        pub struct $id(::std::vec::Vec<$item>);
        header!(@deref $id => Vec<$item>);

        impl $id {
            #[inline]
            pub fn new(values: Vec<$item>) -> ::std::result::Result<$id, $crate::Error> {
                if values.is_empty() {
                    Err($crate::Error::too_few_values())
                } else {
                    Ok($id(values))
                }
            }
        }

        impl $crate::Header for $id {
            #[inline]
            fn name() -> &'static $crate::http::header::HeaderName {
                &$n
            }

            #[inline]
            fn from_values(
                values: &mut $crate::http::header::ValueIter<$crate::http::header::HeaderValue>,
            ) -> ::std::result::Result<::std::option::Option<$id>, $crate::Error>
            {
                match $crate::util::parse_comma_delimited(values)? {
                    Some(values) => $id::new(values).map(Some),
                    None => Ok(None),
                }
            }

            #[inline]
            fn to_values(&self, values: &mut $crate::ToValues) {
                $crate::util::encode_comma_delimited(&self.0, values);
            }
        }

        impl ::std::convert::From<$item> for $id {
            #[inline]
            fn from(value: $item) -> $id {
                $id(vec![value])
            }
        }
    };
    // single value
    ($(#[$a:meta])*($id:ident, $n:expr) => [$value:ty]) => {
        $(#[$a])*
        #[derive(Clone, Debug, PartialEq)]
        pub struct $id(pub $value);
        header!(@deref $id => $value);
        header!(@derefmut  $id => $value);
        impl $crate::Header for $id {
            #[inline]
            fn name() -> &'static $crate::http::header::HeaderName {
                &$n
            }

            #[inline]
            fn from_values(
                values: &mut $crate::http::header::ValueIter<$crate::http::header::HeaderValue>,
            ) -> ::std::result::Result<::std::option::Option<$id>, $crate::Error>
            {
                $crate::util::parse_single_value(values).map(|r| r.map($id))
            }

            #[inline]
            fn to_values(&self, values: &mut $crate::ToValues) {
                $crate::util::encode_single_value(&self.0, values);
            }
        }
    };
    // List header, one or more items with "*" option
    ($(#[$a:meta])*($id:ident, $n:expr) => (Any / ($item:ty)+)) => {
        $(#[$a])*
        #[derive(Clone, Debug, PartialEq)]
        pub enum $id {
            /// Any value is a match
            Any,
            /// Only the listed items are a match
            Items(Vec<$item>),
        }
        impl $crate::Header for $id {
            #[inline]
            fn name() -> &'static $crate::http::header::HeaderName {
                &$n
            }
            #[inline]
            fn from_values(
                values: &mut $crate::http::header::ValueIter<$crate::http::header::HeaderValue>,
            ) -> ::std::result::Result<::std::option::Option<$id>, $crate::Error>
            {
                {
                    let mut pvalues = values.by_ref().peekable();
                    if let Some(first) = pvalues.peek() {
                        if let Ok("*") = first.to_str() {
                            return Ok(Some($id::Any));
                        }
                    }
                }
                match $crate::util::parse_comma_delimited(values)? {
                    Some(values) => Ok(Some($id::Items(values))),
                    None => Ok(None),
                }
            }
            #[inline]
            fn to_values(&self, values: &mut $crate::ToValues) {
                match *self {
                    $id::Any => $crate::util::encode_single_value("*", values),
                    $id::Items(ref fields) => $crate::util::encode_comma_delimited(fields, values),
                }
            }
        }
    };
    (@deref $id:ident => $t:ty) => {
        impl ::std::ops::Deref for $id {
            type Target = $t;

            #[inline]
            fn deref(&self) -> &$t {
                &self.0
            }
        }
    };
    (@derefmut $id:ident => $t:ty) => {
        impl ::std::ops::DerefMut for $id {
            #[inline]
            fn deref_mut(&mut self) -> &mut $t {
                &mut self.0
            }
        }
    };
}

macro_rules! token {
    (
        $(#[$attr:meta])* $name:ident => {
            $(
                $(#[$variant_attr:meta])*
                $variant:ident => $s:expr => [$($alias:expr),*],
            )*
        }
    ) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        #[allow(non_camel_case_types)]
        enum Inner {
            $(
                $variant,
            )*
            Other(String),
        }

        $(#[$attr])*
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $name(Inner);

        impl $name {
            $(
                $(#[$variant_attr])*
                pub const $variant: $name = $name(Inner::$variant);
            )*

            /// Constructs a new instance of this value from a string.
            ///
            /// An error is returned if the string is not a valid token.
            pub fn new(s: &str) -> ::std::result::Result<$name, $crate::Error> {
                $(
                    if s.eq_ignore_ascii_case($s) {
                        return Ok($name(Inner::$variant));
                    }

                    $(
                        if s.eq_ignore_ascii_case($alias) {
                            return Ok($name(Inner::$variant));
                        }
                    )*
                )*

                if $crate::util::is_token(s) {
                    Ok($name(Inner::Other(s.to_ascii_lowercase())))
                } else {
                    Err($crate::Error::invalid_value())
                }
            }

            /// Returns the string representation of this token.
            pub fn as_str(&self) -> &str {
                match self.0 {
                    $(
                        Inner::$variant => $s,
                    )*
                    Inner::Other(ref s) => s,
                }
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                fmt.write_str(self.as_str())
            }
        }

        impl ::std::str::FromStr for $name {
            type Err = $crate::Error;

            fn from_str(s: &str) -> ::std::result::Result<$name, $crate::Error> {
                $name::new(s)
            }
        }
    }
}

mod accept;
mod accept_encoding;
mod access_control_allow_credentials;
mod access_control_allow_headers;
mod access_control_allow_methods;
mod access_control_allow_origin;
mod access_control_expose_headers;
mod access_control_max_age;
mod access_control_request_method;
mod access_control_request_headers;
mod accept_language;
mod allow;
mod auth_scheme;
mod authorization;
mod cache_control;
mod content_coding;
mod content_disposition;
mod content_encoding;
mod content_length;
mod content_location;
mod content_type;
mod content_range;
mod cookie;
mod credentials;
mod etag;
mod expires;
mod host;
mod http_date;
mod if_none_match;
mod if_modified_since;
mod last_modified;
mod location;
mod origin;
mod pragma;
mod proxy_authorization;
mod quality;
mod range;
mod referer;
mod referrer_policy;
mod retry_after;
mod set_cookie;
mod token68;
mod user_agent;
mod vary;
