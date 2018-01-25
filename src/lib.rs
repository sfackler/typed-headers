extern crate http;

use std::mem;

use http::header::{self, HeaderMap, HeaderName, HeaderValue, InvalidHeaderValue};

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
    fn parse<'a>(
        values: &mut header::ValueIter<'a, HeaderValue>,
    ) -> Result<Option<Self>, ParseError>
    where
        Self: Sized;

    /// Serializes the header to raw values.
    ///
    /// Each call to `values.append` adds a header entry. Almost all headers should only append a
    /// single value. `Set-Cookie` is a rare exception.
    fn to_values(&self, values: &mut ToValues) -> Result<(), InvalidHeaderValue>;
}

pub struct ParseError(());

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

pub trait HeaderMapExt {
    /// Retrieves the specified header from the map, if present.
    fn typed_get<H>(&self) -> Result<Option<H>, ParseError>
    where
        H: Header;

    /// Inserts the provided header into the map.
    ///
    /// This overwrites any existing entries for that header.
    fn typed_set<H>(&mut self, header: &H) -> Result<(), InvalidHeaderValue>
    where
        H: Header;
}

impl HeaderMapExt for HeaderMap {
    fn typed_get<H>(&self) -> Result<Option<H>, ParseError>
    where
        H: Header,
    {
        let mut values = self.get_all(H::name()).iter();
        match H::parse(&mut values) {
            Ok(header) => match values.next() {
                Some(_) => Err(ParseError(())),
                None => Ok(header),
            },
            Err(e) => Err(e),
        }
    }

    fn typed_set<H>(&mut self, header: &H) -> Result<(), InvalidHeaderValue>
    where
        H: Header,
    {
        let entry = self.entry(H::name()).unwrap();
        let mut values = ToValues(ToValuesState::First(entry));
        header.to_values(&mut values)
    }
}
