use http::header::{self, HeaderValue};
use std::str::FromStr;

use ParseError;

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
