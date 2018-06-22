use http::header::{self, HeaderName, HeaderValue, CONTENT_LENGTH};
use std::ops::{Deref, DerefMut};

use {util, Error, Header, ToValues};

/// `Content-Length` header, defined in
/// [RFC7230](http://tools.ietf.org/html/rfc7230#section-3.3.2)
///
/// When a message does not have a `Transfer-Encoding` header field, a
/// Content-Length header field can provide the anticipated size, as a
/// decimal number of octets, for a potential payload body.  For messages
/// that do include a payload body, the Content-Length field-value
/// provides the framing information necessary for determining where the
/// body (and message) ends.  For messages that do not include a payload
/// body, the Content-Length indicates the size of the selected
/// representation.
///
/// # ABNF
///
/// ```text
/// Content-Length = 1*DIGIT
/// ```
///
/// # Example values
///
/// * `3495`
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
    fn from_values(
        values: &mut header::ValueIter<HeaderValue>,
    ) -> Result<Option<ContentLength>, Error> {
        let mut length = None;

        for value in values {
            let value = value.to_str().map_err(|_| Error::invalid_value())?;
            if value.trim().is_empty() {
                return Err(Error::invalid_value());
            }

            for elem in value.split(',') {
                let elem = elem.trim();
                if elem.is_empty() {
                    continue;
                }

                let elem = elem.parse().map_err(|_| Error::invalid_value())?;
                match length {
                    Some(length) if length != elem => return Err(Error::invalid_value()),
                    Some(_) => {}
                    None => length = Some(elem),
                }
            }
        }

        Ok(length.map(ContentLength))
    }

    #[inline]
    fn to_values(&self, values: &mut ToValues) {
        util::encode_single_value(&self.0, values);
    }
}
