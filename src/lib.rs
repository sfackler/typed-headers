//! Typed HTTP header serialization and deserialization.
//!
//! This crate is still in its early, experimental stages. It currently takes a fairly pedantic view of parsing, and
//! tries to support exactly what's specified in the HTTP RFCs.
//!
//! The `HeaderMapExt` extension trait provides new methods on the `http::HeaderMap` type to insert, retrieve, and
//! remove headers in a typed manner.
#![doc(html_root_url = "https://docs.rs/typed-headers/0.1")]

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
    fn from_values<'a>(
        values: &mut header::ValueIter<'a, HeaderValue>,
    ) -> Result<Option<Self>, Error>
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

#[derive(Debug)]
enum ErrorKind {
    InvalidValue,
    TooFewValues,
    TooManyValues,
}

/// An error serializing or deserializing a header.
#[derive(Debug)]
pub struct Error(ErrorKind);

impl Error {
    #[inline]
    pub fn invalid_value() -> Error {
        Error(ErrorKind::InvalidValue)
    }

    #[inline]
    pub fn too_few_values() -> Error {
        Error(ErrorKind::TooFewValues)
    }

    #[inline]
    pub fn too_many_values() -> Error {
        Error(ErrorKind::TooManyValues)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let s = match self.0 {
            ErrorKind::InvalidValue => "invalid header value",
            ErrorKind::TooFewValues => "too few header values",
            ErrorKind::TooManyValues => "too many header values",
        };
        fmt.write_str(s)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "header error"
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

    /// Removes and returns the specified header from the map, if present.
    ///
    /// The header will be removed even if it doesn't successfully parse.
    fn typed_remove<H>(&mut self) -> Result<Option<H>, Error>
    where
        H: Header;
}

impl HeaderMapExt for HeaderMap {
    fn typed_get<H>(&self) -> Result<Option<H>, Error>
    where
        H: Header,
    {
        let mut values = self.get_all(H::name()).iter();
        match H::from_values(&mut values) {
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

    fn typed_remove<H>(&mut self) -> Result<Option<H>, Error>
    where
        H: Header,
    {
        match self.entry(H::name()).unwrap() {
            header::Entry::Occupied(entry) => {
                let r = H::from_values(&mut entry.iter());
                entry.remove();
                r
            }
            header::Entry::Vacant(_) => Ok(None),
        }
    }
}
