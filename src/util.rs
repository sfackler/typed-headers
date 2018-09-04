use http::header::{self, HeaderMap, HeaderName, HeaderValue};
use impls;
use http;
use mime::Mime;
use std::error;
use std::fmt::{self, Write};
use std::str::FromStr;
use impls::{CacheDirective, HttpDate, ContentRangeSpec};
use shared::{EntityTag, Charset};
use language_tags::LanguageTag;
use percent_encoding;

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

impl ToHeaderValue for u32 {
    fn to_header_value(&self) -> String {
        self.to_string()
    }
}

impl ToHeaderValue for HttpDate {
    fn to_header_value(&self) -> String {
        self.to_string()
    }
}

impl ToHeaderValue for ContentRangeSpec {
    fn to_header_value(&self) -> String {
        self.to_string()
    }
}

impl ToHeaderValue for CacheDirective {
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

impl ToHeaderValue for EntityTag {
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

pub fn parse_comma_delimited<'b, 'a: 'b, T, I>(
    values: &'b mut I,
) -> Result<Option<Vec<T>>, Error>
where
    T: FromStr,
    T::Err: Into<Box<error::Error + Sync + Send>>,
    I: Iterator<Item=&'a HeaderValue> + 'b,
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


/// An extended header parameter value (i.e., tagged with a character set and optionally,
/// a language), as defined in [RFC 5987](https://tools.ietf.org/html/rfc5987#section-3.2).
#[derive(Clone, Debug, PartialEq)]
pub struct ExtendedValue {
    /// The character set that is used to encode the `value` to a string.
    pub charset: Charset,
    /// The human language details of the `value`, if available.
    pub language_tag: Option<LanguageTag>,
    /// The parameter value, as expressed in octets.
    pub value: Vec<u8>,
}

/// Parses extended header parameter values (`ext-value`), as defined in
/// [RFC 5987](https://tools.ietf.org/html/rfc5987#section-3.2).
///
/// Extended values are denoted by parameter names that end with `*`.
///
/// ## ABNF
///
/// ```text
/// ext-value     = charset  "'" [ language ] "'" value-chars
///               ; like RFC 2231's <extended-initial-value>
///               ; (see [RFC2231], Section 7)
///
/// charset       = "UTF-8" / "ISO-8859-1" / mime-charset
///
/// mime-charset  = 1*mime-charsetc
/// mime-charsetc = ALPHA / DIGIT
///               / "!" / "#" / "$" / "%" / "&"
///               / "+" / "-" / "^" / "_" / "`"
///               / "{" / "}" / "~"
///               ; as <mime-charset> in Section 2.3 of [RFC2978]
///               ; except that the single quote is not included
///               ; SHOULD be registered in the IANA charset registry
///
/// language      = <Language-Tag, defined in [RFC5646], Section 2.1>
///
/// value-chars   = *( pct-encoded / attr-char )
///
/// pct-encoded   = "%" HEXDIG HEXDIG
///               ; see [RFC3986], Section 2.1
///
/// attr-char     = ALPHA / DIGIT
///               / "!" / "#" / "$" / "&" / "+" / "-" / "."
///               / "^" / "_" / "`" / "|" / "~"
///               ; token except ( "*" / "'" / "%" )
/// ```
pub fn parse_extended_value(val: &str) -> Result<ExtendedValue, Error> {

    // Break into three pieces separated by the single-quote character
    let mut parts = val.splitn(3,'\'');

    // Interpret the first piece as a Charset
    let charset: Charset = match parts.next() {
        None => return Err(Error::invalid_value()),
        Some(n) => try!(FromStr::from_str(n)),
    };

    // Interpret the second piece as a language tag
    let lang: Option<LanguageTag> = match parts.next() {
        None => return Err(Error::invalid_value()),
        Some("") => None,
        Some(s) => match s.parse() {
            Ok(lt) => Some(lt),
            Err(_) => return Err(Error::invalid_value()),
        }
    };

    // Interpret the third piece as a sequence of value characters
    let value: Vec<u8> = match parts.next() {
        None => return Err(Error::invalid_value()),
        Some(v) => percent_encoding::percent_decode(v.as_bytes()).collect(),
    };

    Ok(ExtendedValue {
        charset: charset,
        language_tag: lang,
        value: value,
    })
}

pub fn http_percent_encode(f: &mut fmt::Formatter, bytes: &[u8]) -> fmt::Result {
    let encoded = percent_encoding::percent_encode(bytes, self::percent_encoding_http::HTTP_VALUE);
    fmt::Display::fmt(&encoded, f)
}

mod percent_encoding_http {
    use percent_encoding;

    // internal module because macro is hard-coded to make a public item
    // but we don't want to public export this item
    define_encode_set! {
        // This encode set is used for HTTP header values and is defined at
        // https://tools.ietf.org/html/rfc5987#section-3.2
        pub HTTP_VALUE = [percent_encoding::SIMPLE_ENCODE_SET] | {
            ' ', '"', '%', '\'', '(', ')', '*', ',', '/', ':', ';', '<', '-', '>', '?',
            '[', '\\', ']', '{', '}'
        }
    }
}
