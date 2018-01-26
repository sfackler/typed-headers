use http::header::{self, HeaderValue, InvalidHeaderValue};
use std::str::FromStr;
use std::fmt::{self, Write};

use {ParseError, ToValues};

pub fn from_one_str<T>(values: &mut header::ValueIter<HeaderValue>) -> Result<Option<T>, ParseError>
where
    T: FromStr<Err = ParseError>,
{
    match values.next() {
        Some(value) => {
            let value = value.to_str()?.trim().parse()?;
            Ok(Some(value))
        }
        None => Ok(None),
    }
}

pub fn parse_comma_delimited<T>(
    values: &mut header::ValueIter<HeaderValue>,
) -> Result<Option<Vec<T>>, ParseError>
where
    T: FromStr<Err = ParseError>,
{
    let mut out = vec![];
    let mut empty = true;
    for value in values {
        empty = false;

        let value = value.to_str()?;
        for elem in value.split(',') {
            let elem = elem.trim();
            if elem.is_empty() {
                continue;
            }

            let elem = elem.parse()?;
            out.push(elem);
        }
    }

    if empty {
        Ok(None)
    } else {
        Ok(Some(out))
    }
}

pub fn encode_comma_delimited<I>(
    elements: I,
    values: &mut ToValues,
) -> Result<(), InvalidHeaderValue>
where
    I: IntoIterator,
    I::Item: fmt::Display,
{
    let mut out = String::new();
    let mut it = elements.into_iter();
    if let Some(elem) = it.next() {
        write!(out, "{}", elem).unwrap();
    }
    for elem in it {
        write!(out, ",{}", elem).unwrap();
    }
    let value = HeaderValue::from_str(&out)?;
    values.append(value);
    Ok(())
}
