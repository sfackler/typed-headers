use http::header::{self, HeaderName, HeaderValue, CONTENT_LENGTH};
use std::ops::{Deref, DerefMut};

use {util, Error, Header, ToValues};

#[derive(Clone, Debug, PartialEq)]
pub struct ContentLength(pub u64);

impl Deref for ContentLength {
    type Target = u64;

    #[inline]
    fn deref(&self) -> &u64 {
        &self.0
    }
}

impl DerefMut for ContentLength {
    #[inline]
    fn deref_mut(&mut self) -> &mut u64 {
        &mut self.0
    }
}

impl Header for ContentLength {
    #[inline]
    fn name() -> &'static HeaderName {
        &CONTENT_LENGTH
    }

    // RFC 7230 permits multiple identical copies of Content-Length, and there apparently exist
    // implementations that produce that!
    // https://github.com/request/request/issues/2091#issuecomment-328715113
    #[inline]
    fn parse(values: &mut header::ValueIter<HeaderValue>) -> Result<Option<ContentLength>, Error> {
        let mut length = None;

        for value in values {
            let value = value.to_str().map_err(Error::new)?;
            if value.trim().is_empty() {
                return Err(Error::new("empty header value"));
            }

            for elem in value.split(',') {
                let elem = elem.trim();
                if elem.is_empty() {
                    continue;
                }

                let elem = elem.parse().map_err(Error::new)?;
                match length {
                    Some(length) if length != elem => {
                        return Err(Error::new("multiple non-identical Content-Length headers"));
                    }
                    Some(_) => {}
                    None => length = Some(elem),
                }
            }
        }

        Ok(length.map(ContentLength))
    }

    #[inline]
    fn to_values(&self, values: &mut ToValues) -> Result<(), Error> {
        util::encode_single_value(&self.0, values)
    }
}
