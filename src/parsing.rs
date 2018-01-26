use bytes::BytesMut;
use http::header::{self, HeaderValue};
use std::error;
use std::fmt::{self, Write};
use std::str::FromStr;

use {Error, ToValues};

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
                .map_err(Error::new)?
                .trim()
                .parse()
                .map_err(Error::new)?;
            Ok(Some(value))
        }
        None => Ok(None),
    }
}

pub fn encode_single_value<T>(value: &T, values: &mut ToValues) -> Result<(), Error>
where
    T: fmt::Display,
{
    let mut buf = BytesMut::new();
    write!(buf, "{}", value).unwrap();
    let value = HeaderValue::from_shared(buf.freeze()).map_err(Error::new)?;
    values.append(value);
    Ok(())
}

pub fn parse_comma_delimited<T>(
    values: &mut header::ValueIter<HeaderValue>,
    min: Option<usize>,
    max: Option<usize>,
) -> Result<Option<Vec<T>>, Error>
where
    T: FromStr,
    T::Err: Into<Box<error::Error + Sync + Send>>,
{
    let mut out = vec![];
    let mut empty = true;
    for value in values {
        empty = false;

        let value = value.to_str().map_err(Error::new)?;
        for elem in value.split(',') {
            let elem = elem.trim();
            if elem.is_empty() {
                continue;
            }

            let elem = elem.parse().map_err(Error::new)?;
            out.push(elem);
        }
    }

    if empty {
        Ok(None)
    } else {
        if let Some(min) = min {
            if out.len() < min {
                return Err(Error::new(format!(
                    "expected at least {} values, but got {}",
                    min,
                    out.len()
                )));
            }
        }
        if let Some(max) = max {
            if out.len() > max {
                return Err(Error::new(format!(
                    "expected at most {} values, but got {}",
                    max,
                    out.len()
                )));
            }
        }

        Ok(Some(out))
    }
}

pub fn encode_comma_delimited<I>(
    elements: I,
    values: &mut ToValues,
    min: Option<usize>,
    max: Option<usize>,
) -> Result<(), Error>
where
    I: IntoIterator,
    I::Item: fmt::Display,
{
    let mut out = String::new();
    let mut it = elements.into_iter();
    let mut count = 0;
    if let Some(elem) = it.next() {
        write!(out, "{}", elem).unwrap();
        count += 1;
    }
    for elem in it {
        write!(out, ",{}", elem).unwrap();
        count += 1;
    }
    if let Some(min) = min {
        if count < min {
            return Err(Error::new(format!(
                "expected at least {} values, but got {}",
                min, count
            )));
        }
    }
    if let Some(max) = max {
        if count > max {
            return Err(Error::new(format!(
                "expected at most {} values, but got {}",
                max, count
            )));
        }
    }
    if count != 0 && out.as_bytes().iter().filter(|&&b| b == b',').count() != count - 1 {
        return Err(Error::new("values contained internal `,` characters"));
    }
    if out.contains(",,") {
        return Err(Error::new("empty values are not permitted"));
    }
    let value = HeaderValue::from_str(&out).map_err(Error::new)?;
    values.append(value);
    Ok(())
}
