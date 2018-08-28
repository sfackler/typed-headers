use http::header::{self, HeaderMap, HeaderName, HeaderValue};
use impls;
use http;
use mime::Mime;
use std::error;
use std::fmt::{self, Write};
use std::str::FromStr;

use {Error, Header, HeaderMapExt, ToValues};

#[inline]
pub fn is_token(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    s.as_bytes().iter().all(|b| match *b {
        b'a'...b'z'
        | b'A'...b'Z'
        | b'0'...b'9'
        | b'!'
        | b'#'
        | b'$'
        | b'%'
        | b'&'
        | b'\''
        | b'*'
        | b'+'
        | b'-'
        | b'.'
        | b'^'
        | b'_'
        | b'`'
        | b'|'
        | b'~' => true,
        _ => false,
    })
}

pub fn parse_single_value<T>(
    values: &mut header::ValueIter<HeaderValue>,
) -> Result<Option<T>, Error>
where
    T: FromStr,
    T::Err: Into<Box<error::Error + Sync + Send>>,
{
    match values.next() {
        Some(value) => {
            let value = value
                .to_str()
                .map_err(|_| Error::invalid_value())?
                .trim()
                .parse()
                .map_err(|_| Error::invalid_value())?;
            Ok(Some(value))
        }
        None => Ok(None),
    }
}

pub trait ToHeaderValue {
    fn to_header_value(&self) -> String;
}

impl<'a, T: ToHeaderValue> ToHeaderValue for &'a T {
    fn to_header_value(&self) -> String {
        (*self).to_header_value()
    }
}

impl ToHeaderValue for String {
    fn to_header_value(&self) -> String {
        self.to_string()
    }
}

impl ToHeaderValue for u64 {
    fn to_header_value(&self) -> String {
        self.to_string()
    }
}

impl ToHeaderValue for impls::Credentials {
    fn to_header_value(&self) -> String {
        self.to_string()
    }
}

impl ToHeaderValue for http::Method {
    fn to_header_value(&self) -> String {
        self.to_string()
    }
}

impl<T: fmt::Display> ToHeaderValue for impls::QualityItem<T> {
    fn to_header_value(&self) -> String {
        self.to_string()
    }
}

impl ToHeaderValue for impls::ContentCoding {
    fn to_header_value(&self) -> String {
        self.to_string()
    }
}

impl ToHeaderValue for Mime {
    fn to_header_value(&self) -> String {
        self.to_string()
    }
}

impl ToHeaderValue for HeaderName {
    fn to_header_value(&self) -> String {
        self.as_str().to_string()
    }
}

impl ToHeaderValue for str {
    fn to_header_value(&self) -> String {
        self.to_string()
    }
}

pub fn encode_single_value<T>(value: &T, values: &mut ToValues)
where
    T: ToHeaderValue + ?Sized
{
    let value = value.to_header_value();
    let value = HeaderValue::from_str(&value).expect("failed to encode header");
    values.append(value);
}

pub fn parse_comma_delimited<T>(
    values: &mut header::ValueIter<HeaderValue>,
) -> Result<Option<Vec<T>>, Error>
where
    T: FromStr,
    T::Err: Into<Box<error::Error + Sync + Send>>,
{
    let mut out = vec![];
    let mut empty = true;
    for value in values {
        empty = false;

        let value = value.to_str().map_err(|_| Error::invalid_value())?;
        for elem in value.split(',') {
            let elem = elem.trim();
            if elem.is_empty() {
                continue;
            }

            let elem = elem.parse().map_err(|_| Error::invalid_value())?;
            out.push(elem);
        }
    }

    if empty {
        Ok(None)
    } else {
        Ok(Some(out))
    }
}

pub fn encode_comma_delimited<I>(elements: I, values: &mut ToValues)
where
    I: IntoIterator,
    I::Item: ToHeaderValue
{
    let mut out = String::new();
    let mut it = elements.into_iter();
    if let Some(elem) = it.next() {
        write!(out, "{}", elem.to_header_value()).unwrap();

        for elem in it {
            write!(out, ", {}", elem.to_header_value()).unwrap();
        }
    }
    let value = HeaderValue::from_str(&out).expect("failed to encode header");
    values.append(value);
}

pub fn test_decode<H>(values: &[&str], expected: &H)
where
    H: Header + PartialEq + fmt::Debug,
{
    let mut map = HeaderMap::new();
    for value in values {
        let value = HeaderValue::from_str(value).unwrap();
        map.append(H::name().clone(), value);
    }

    let header = map.typed_get::<H>().unwrap().unwrap();
    assert_eq!(&header, expected);
}

pub fn test_encode<H>(header: &H, expected: &[&str])
where
    H: Header,
{
    let mut map = HeaderMap::new();
    map.typed_insert(header);

    let values = map.get_all(H::name()).iter().collect::<Vec<_>>();
    assert_eq!(values.len(), expected.len());
    for (value, expected) in values.iter().zip(expected) {
        assert_eq!(value, expected);
    }
}

pub fn test_round_trip<H>(header: &H, expected: &[&str])
where
    H: Header + PartialEq + fmt::Debug,
{
    let mut map = HeaderMap::new();
    map.typed_insert(header);

    let values = map.get_all(H::name()).iter().collect::<Vec<_>>();
    assert_eq!(values.len(), expected.len());
    for (value, expected) in values.iter().zip(expected) {
        assert_eq!(value, expected);
    }

    let actual = map.typed_get::<H>().unwrap().unwrap();
    assert_eq!(header, &actual);
}
