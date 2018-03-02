pub use impls::accept::*;
pub use impls::accept_charset::*;
pub use impls::accept_encoding::*;
pub use impls::accept_ranges::*;
pub use impls::allow::*;
pub use impls::charset::*;
pub use impls::content_coding::*;
pub use impls::content_encoding::*;
pub use impls::content_length::*;
pub use impls::content_type::*;
pub use impls::host::*;
pub use impls::quality::*;
pub use impls::range_unit::*;

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

mod accept;
mod accept_charset;
mod accept_encoding;
mod accept_ranges;
mod allow;
mod charset;
mod content_coding;
mod content_encoding;
mod content_length;
mod content_type;
mod host;
mod quality;
mod range_unit;
