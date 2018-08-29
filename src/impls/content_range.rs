use std::fmt::{self, Display};
use std::str::FromStr;
use http::header::CONTENT_RANGE;
use Error;

header! {
    /// `Content-Range` header, defined in
    /// [RFC7233](http://tools.ietf.org/html/rfc7233#section-4.2)
    (ContentRange, CONTENT_RANGE) => [ContentRangeSpec]
}


/// Content-Range, described in [RFC7233](https://tools.ietf.org/html/rfc7233#section-4.2)
///
/// # ABNF
///
/// ```text
/// Content-Range       = byte-content-range
///                     / other-content-range
///
/// byte-content-range  = bytes-unit SP
///                       ( byte-range-resp / unsatisfied-range )
///
/// byte-range-resp     = byte-range "/" ( complete-length / "*" )
/// byte-range          = first-byte-pos "-" last-byte-pos
/// unsatisfied-range   = "*/" complete-length
///
/// complete-length     = 1*DIGIT
///
/// other-content-range = other-range-unit SP other-range-resp
/// other-range-resp    = *CHAR
/// ```
#[derive(PartialEq, Clone, Debug)]
pub enum ContentRangeSpec {
    /// Byte range
    Bytes {
        /// First and last bytes of the range, omitted if request could not be
        /// satisfied
        range: Option<(u64, u64)>,

        /// Total length of the instance, can be omitted if unknown
        instance_length: Option<u64>
    },

    /// Custom range, with unit not registered at IANA
    Unregistered {
        /// other-range-unit
        unit: String,

        /// other-range-resp
        resp: String
    }
}

fn split_in_two(s: &str, separator: char) -> Option<(&str, &str)> {
    let mut iter = s.splitn(2, separator);
    match (iter.next(), iter.next()) {
        (Some(a), Some(b)) => Some((a, b)),
        _ => None
    }
}

impl FromStr for ContentRangeSpec {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        let res = match split_in_two(s, ' ') {
            Some(("bytes", resp)) => {
                let (range, instance_length) = try!(split_in_two(resp, '/').ok_or(Error::invalid_value()));

                let instance_length = if instance_length == "*" {
                    None
                } else {
                    Some(try!(instance_length.parse().map_err(|_| Error::invalid_value())))
                };

                let range = if range == "*" {
                    None
                } else {
                    let (first_byte, last_byte) = try!(split_in_two(range, '-').ok_or(Error::invalid_value()));
                    let first_byte = try!(first_byte.parse().map_err(|_| Error::invalid_value()));
                    let last_byte = try!(last_byte.parse().map_err(|_| Error::invalid_value()));
                    if last_byte < first_byte {
                        return Err(Error::invalid_value());
                    }
                    Some((first_byte, last_byte))
                };

                ContentRangeSpec::Bytes {
                    range: range,
                    instance_length: instance_length
                }
            }
            Some((unit, resp)) => {
                ContentRangeSpec::Unregistered {
                    unit: unit.to_owned(),
                    resp: resp.to_owned()
                }
            }
            _ => return Err(Error::invalid_value())
        };
        Ok(res)
    }
}

impl Display for ContentRangeSpec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ContentRangeSpec::Bytes { range, instance_length } => {
                try!(f.write_str("bytes "));
                match range {
                    Some((first_byte, last_byte)) => {
                        try!(write!(f, "{}-{}", first_byte, last_byte));
                    },
                    None => {
                        try!(f.write_str("*"));
                    }
                };
                try!(f.write_str("/"));
                if let Some(v) = instance_length {
                    write!(f, "{}", v)
                } else {
                    f.write_str("*")
                }
            }
            ContentRangeSpec::Unregistered { ref unit, ref resp } => {
                try!(f.write_str(&unit));
                try!(f.write_str(" "));
                f.write_str(resp)
            }
        }
    }
}
