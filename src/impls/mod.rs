pub use impls::content_coding::*;
pub use impls::content_encoding::*;
pub use impls::content_length::*;
pub use impls::host::*;

macro_rules! header {
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
                $crate::parsing::parse_comma_delimited(values, Some(1), None).map(|r| r.map($id))
            }

            #[inline]
            fn to_values(
                &self,
                values: &mut $crate::ToValues,
            ) -> ::std::result::Result<(), $crate::Error>
            {
                $crate::parsing::encode_comma_delimited(&self.0, values, Some(1), None)
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
                $crate::parsing::parse_single_value(values).map(|r| r.map($id))
            }

            #[inline]
            fn to_values(
                &self,
                values: &mut $crate::ToValues,
            ) -> ::std::result::Result<(), $crate::Error>
            {
                $crate::parsing::encode_single_value(&self.0, values)
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

mod content_coding;
mod content_encoding;
mod content_length;
mod host;
