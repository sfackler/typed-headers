pub use self::accept::Accept;
pub use self::accept_language::AcceptLanguage;
pub use self::accept_encoding::AcceptEncoding;
pub use self::allow::Allow;
pub use self::auth_scheme::AuthScheme;
pub use self::authorization::Authorization;
pub use self::content_coding::ContentCoding;
pub use self::content_encoding::ContentEncoding;
pub use self::content_length::ContentLength;
pub use self::content_type::ContentType;
pub use self::credentials::Credentials;
pub use self::host::Host;
pub use self::http_date::HttpDate;
pub use self::proxy_authorization::ProxyAuthorization;
pub use self::quality::{Quality, QualityItem};
pub use self::retry_after::RetryAfter;
pub use self::token68::Token68;

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
mod accept_language;
mod allow;
mod auth_scheme;
mod authorization;
mod content_coding;
mod content_encoding;
mod content_length;
mod content_type;
mod credentials;
mod host;
mod http_date;
mod proxy_authorization;
mod quality;
mod retry_after;
mod token68;
