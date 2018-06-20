use chrono::{DateTime, NaiveDateTime, Utc};
use std::fmt;
use std::str::FromStr;
use std::time::SystemTime;

use Error;

const IMF_FIXDATE_PATTERN: &'static str = "%a, %d %b %Y %T GMT";
const RFC850_DATE_PATTERN: &'static str = "%A, %d-%b-%y %T GMT";
const ASCTIME_DATE_PATTERN: &'static str = "%a %b %e %T %Y";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpDate(DateTime<Utc>);

impl From<SystemTime> for HttpDate {
    #[inline]
    fn from(t: SystemTime) -> HttpDate {
        HttpDate(DateTime::from(t))
    }
}

impl From<HttpDate> for SystemTime {
    #[inline]
    fn from(t: HttpDate) -> SystemTime {
        SystemTime::from(t.0)
    }
}

impl FromStr for HttpDate {
    type Err = Error;

    fn from_str(s: &str) -> Result<HttpDate, Error> {
        let naive = NaiveDateTime::parse_from_str(s, IMF_FIXDATE_PATTERN)
            .or_else(|_| NaiveDateTime::parse_from_str(s, RFC850_DATE_PATTERN))
            .or_else(|_| NaiveDateTime::parse_from_str(s, ASCTIME_DATE_PATTERN))
            .map_err(|_| Error::invalid_value())?;

        Ok(HttpDate(DateTime::from_utc(naive, Utc)))
    }
}

impl fmt::Display for HttpDate {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0.format(IMF_FIXDATE_PATTERN), fmt)
    }
}
