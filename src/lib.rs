extern crate http;

use std::mem;

use http::header::{self, HeaderMap, HeaderName, HeaderValue, InvalidHeaderValue};

pub trait Header {
    fn name() -> &'static HeaderName;

    fn parse<'a>(values: header::GetAll<'a, HeaderValue>) -> Result<Option<Self>, ParseError>
    where
        Self: Sized;

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
    fn typed_get<H>(&self) -> Result<Option<H>, ParseError>
    where
        H: Header;

    fn typed_set<H>(&mut self, header: &H) -> Result<(), InvalidHeaderValue>
    where
        H: Header;
}

impl HeaderMapExt for HeaderMap {
    fn typed_get<H>(&self) -> Result<Option<H>, ParseError>
    where
        H: Header,
    {
        H::parse(self.get_all(H::name()))
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
