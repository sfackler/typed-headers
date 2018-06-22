use http::header::{HeaderName, HeaderValue, ValueIter, RETRY_AFTER};

use {Error, Header, HttpDate, ToValues};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RetryAfter {
    DelaySeconds(u64),
    HttpDate(HttpDate),
}

impl Header for RetryAfter {
    fn name() -> &'static HeaderName {
        &RETRY_AFTER
    }

    #[inline]
    fn from_values<'a>(
        values: &mut ValueIter<'a, HeaderValue>,
    ) -> Result<Option<RetryAfter>, Error> {
        let value = match values.next() {
            Some(value) => value,
            None => return Ok(None),
        };

        let value = value.to_str().map_err(|_| Error::invalid_value())?;

        value
            .parse::<u64>()
            .ok()
            .map(RetryAfter::DelaySeconds)
            .or_else(|| value.parse::<HttpDate>().ok().map(RetryAfter::HttpDate))
            .map(Some)
            .ok_or_else(|| Error::invalid_value())
    }

    #[inline]
    fn to_values(&self, values: &mut ToValues) {
        let s = match *self {
            RetryAfter::DelaySeconds(delay) => delay.to_string(),
            RetryAfter::HttpDate(ref date) => date.to_string(),
        };
        let value = HeaderValue::from_str(&s).expect("retry-after should be valid");
        values.append(value);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use util;

    #[test]
    fn rfc1() {
        util::test_round_trip(&RetryAfter::DelaySeconds(120), &["120"]);
    }

    #[test]
    fn rfc2() {
        util::test_round_trip(
            &RetryAfter::HttpDate("Fri, 31 Dec 1999 23:59:59 GMT".parse().unwrap()),
            &["Fri, 31 Dec 1999 23:59:59 GMT"],
        );
    }

    #[test]
    fn rfc850() {
        util::test_decode(
            &["Sunday, 06-Nov-94 08:49:37 GMT"],
            &RetryAfter::HttpDate("Sun, 06 Nov 1994 08:49:37 GMT".parse().unwrap()),
        );
    }

    #[test]
    fn asctime() {
        util::test_decode(
            &["Sun Nov  6 08:49:37 1994"],
            &RetryAfter::HttpDate("Sun, 06 Nov 1994 08:49:37 GMT".parse().unwrap()),
        );
    }
}
