//! Typed HTTP header serialization and deserialization supported.

extern crate base64;
extern crate bytes;
extern crate chrono;

pub extern crate http;
pub extern crate mime;

use http::header::{self, HeaderMap, HeaderName, HeaderValue};
use std::error;
use std::fmt;
use std::mem;

pub use impls::*;

mod impls;
pub mod util;

pub trait Header {
    /// Returns the name of this header.
    ///
    /// The `http` crate provides constants for all standard header names. Implementations for
    /// nonstandard headers can use the `lazy_static` crate to obtain a static reference to a
    /// `HeaderName`.
    fn name() -> &'static HeaderName;

    /// Parses the header from the raw value bytes.
    ///
    /// The iterator may be empty, which indicates that the header was not present, and `Ok(None)`
    /// should be returned.
    ///
    /// If the iterator is not exhausted when this function returns, it will be treated as a parse
    /// error.
    fn parse<'a>(values: &mut header::ValueIter<'a, HeaderValue>) -> Result<Option<Self>, Error>
    where
        Self: Sized;

    /// Serializes the header to raw values.
    ///
    /// Each call to `values.append` adds a header entry. Almost all headers should only append a
    /// single value. `Set-Cookie` is a rare exception.
    ///
    /// This method is infallible. Header implementations should ensure at construction time that
    /// they will be able to successfully serialize.
    fn to_values(&self, values: &mut ToValues);
}

/// An error serializing or deserializing a header.
#[derive(Debug)]
pub struct Error(Box<error::Error + Sync + Send>);

impl Error {
    /// Creates a new error with the provided cause.
    pub fn custom<E>(e: E) -> Error
    where
        E: Into<Box<error::Error + Sync + Send>>,
    {
        Error(e.into())
    }

    fn too_many_values() -> Error {
        Error::custom("too many header values")
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, fmt)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        error::Error::description(&*self.0)
    }
}

enum ToValuesState<'a> {
    First(header::Entry<'a, HeaderValue>),
    Latter(header::OccupiedEntry<'a, HeaderValue>),
    Tmp,
}

pub struct ToValues<'a>(ToValuesState<'a>);

impl<'a> ToValues<'a> {
    pub fn append(&mut self, value: HeaderValue) {
        let entry = match mem::replace(&mut self.0, ToValuesState::Tmp) {
            ToValuesState::First(header::Entry::Occupied(mut e)) => {
                e.insert(value);
                e
            }
            ToValuesState::First(header::Entry::Vacant(e)) => e.insert_entry(value),
            ToValuesState::Latter(mut e) => {
                e.append(value);
                e
            }
            ToValuesState::Tmp => unreachable!(),
        };
        self.0 = ToValuesState::Latter(entry);
    }
}

/// An extension trait adding typed getters and setters to `HeaderMap`.
pub trait HeaderMapExt {
    /// Retrieves the specified header from the map, if present.
    fn typed_get<H>(&self) -> Result<Option<H>, Error>
    where
        H: Header;

    /// Inserts the provided header into the map.
    ///
    /// This overwrites any existing entries for that header.
    fn typed_insert<H>(&mut self, header: &H)
    where
        H: Header;
}

impl HeaderMapExt for HeaderMap {
    fn typed_get<H>(&self) -> Result<Option<H>, Error>
    where
        H: Header,
    {
        let mut values = self.get_all(H::name()).iter();
        match H::parse(&mut values) {
            Ok(header) => match values.next() {
                Some(_) => Err(Error::too_many_values()),
                None => Ok(header),
            },
            Err(e) => Err(e),
        }
    }

    fn typed_insert<H>(&mut self, header: &H)
    where
        H: Header,
    {
        let entry = self.entry(H::name()).unwrap();
        let mut values = ToValues(ToValuesState::First(entry));
        header.to_values(&mut values);
    }
}
