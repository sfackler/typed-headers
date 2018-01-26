pub use impls::content_encoding::*;
pub use impls::encoding::*;
pub use impls::host::*;

macro_rules! header {
    // 1#rule
    ($(#[$a:meta])*($id:ident, $n:expr) => ($item:ty)+) => {
        $(#[$a])*
        #[derive(Clone, Debug, PartialEq)]
        pub struct $id(pub Vec<$item>);
        header!(@deref $id => Vec<$item>);
        impl $crate::Header for $id {
            #[inline]
            fn name() -> &'static $crate::http::header::HeaderName {
                &$n
            }

            #[inline]
            fn parse(
                values: &mut $crate::http::header::ValueIter<$crate::http::header::HeaderValue>,
            ) -> ::std::result::Result<::std::option::Option<$id>, $crate::ParseError>
            {
                let values = match $crate::parsing::parse_comma_delimited(values)? {
                    ::std::option::Option::Some(values) => values,
                    ::std::option::Option::None => return Ok(::std::option::Option::None),
                };

                if values.is_empty() {
                    ::std::result::Result::Err($crate::ParseError::empty_list())
                } else {
                    ::std::result::Result::Ok(::std::option::Option::Some($id(values)))
                }
            }

            #[inline]
            fn to_values(
                &self,
                values: &mut $crate::ToValues,
            ) -> ::std::result::Result<(), $crate::http::header::InvalidHeaderValue>
            {
                $crate::parsing::encode_comma_delimited(&self.0, values)
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

mod content_encoding;
mod encoding;
mod host;
