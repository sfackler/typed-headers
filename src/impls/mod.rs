pub use impls::accept::*;
pub use impls::accept_encoding::*;
pub use impls::content_coding::*;
pub use impls::content_encoding::*;
pub use impls::content_length::*;
pub use impls::content_type::*;
pub use impls::quality::*;

macro_rules! header {
    // #rule
    ($(#[$a:meta])*($id:ident, $n:expr) => ($item:ty)*) => {
        $(#[$a])*
        #[derive(Clone, Debug, PartialEq)]
        pub struct $id(pub ::std::vec::Vec<$item>);
        header!(@deref $id => Vec<$item>);
        impl $crate::Header for $id {
            #[inline]
            fn name() -> &'static $crate::http::header::HeaderName {
                &$n
            }

            #[inline]
            fn parse(
                values: &mut $crate::http::header::ValueIter<$crate::http::header::HeaderValue>,
            ) -> ::std::result::Result<::std::option::Option<$id>, $crate::Error>
            {
                $crate::util::parse_comma_delimited(values, None, None).map(|r| r.map($id))
            }

            #[inline]
            fn to_values(
                &self,
                values: &mut $crate::ToValues,
            ) -> ::std::result::Result<(), $crate::Error>
            {
                $crate::util::encode_comma_delimited(&self.0, values, None, None)
            }
        }
    };
    // 1#rule
    ($(#[$a:meta])*($id:ident, $n:expr) => ($item:ty)+) => {
        $(#[$a])*
        #[derive(Clone, Debug, PartialEq)]
        pub struct $id(pub ::std::vec::Vec<$item>);
        header!(@deref $id => Vec<$item>);
        impl $crate::Header for $id {
            #[inline]
            fn name() -> &'static $crate::http::header::HeaderName {
                &$n
            }

            #[inline]
            fn parse(
                values: &mut $crate::http::header::ValueIter<$crate::http::header::HeaderValue>,
            ) -> ::std::result::Result<::std::option::Option<$id>, $crate::Error>
            {
                $crate::util::parse_comma_delimited(values, Some(1), None).map(|r| r.map($id))
            }

            #[inline]
            fn to_values(
                &self,
                values: &mut $crate::ToValues,
            ) -> ::std::result::Result<(), $crate::Error>
            {
                $crate::util::encode_comma_delimited(&self.0, values, Some(1), None)
            }
        }
    };
    // single value
    ($(#[$a:meta])*($id:ident, $n:expr) => [$value:ty]) => {
        $(#[$a])*
        #[derive(Clone, Debug, PartialEq)]
        pub struct $id(pub $value);
        header!(@deref $id => $value);
        impl $crate::Header for $id {
            #[inline]
            fn name() -> &'static $crate::http::header::HeaderName {
                &$n
            }

            #[inline]
            fn parse(
                values: &mut $crate::http::header::ValueIter<$crate::http::header::HeaderValue>,
            ) -> ::std::result::Result<::std::option::Option<$id>, $crate::Error>
            {
                $crate::util::parse_single_value(values).map(|r| r.map($id))
            }

            #[inline]
            fn to_values(
                &self,
                values: &mut $crate::ToValues,
            ) -> ::std::result::Result<(), $crate::Error>
            {
                $crate::util::encode_single_value(&self.0, values)
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
        $(#[$attr:meta])* $name:ident, $error:ident => {
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
            pub fn new(s: &str) -> ::std::result::Result<$name, $error> {
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
                    Err($error(()))
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
            type Err = $error;

            fn from_str(s: &str) -> ::std::result::Result<$name, $error> {
                $name::new(s)
            }
        }

        #[derive(Debug)]
        pub struct $error(());

        impl ::std::fmt::Display for $error {
            fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                fmt.write_str("invalid token")
            }
        }

        impl ::std::error::Error for $error {
            fn description(&self) -> &str {
                "invalid token"
            }
        }
    }
}

mod accept;
mod accept_encoding;
mod content_coding;
mod content_encoding;
mod content_length;
mod content_type;
mod quality;
