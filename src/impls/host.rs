use http::header::{self, HeaderName, HeaderValue, HOST};
use std::error;
use std::fmt;
use std::str::FromStr;

use {util, Error, Header, ToValues};

pub struct Host {
    hostname: String,
    port: Option<u16>,
}

impl Host {
    pub fn new(hostname: &str, port: Option<u16>) -> Host {
        Host {
            hostname: hostname.to_string(),
            port,
        }
    }

    pub fn hostname(&self) -> &str {
        &self.hostname
    }

    pub fn port(&self) -> Option<u16> {
        self.port
    }
}

impl Header for Host {
    fn name() -> &'static HeaderName {
        &HOST
    }

    fn parse(values: &mut header::ValueIter<HeaderValue>) -> Result<Option<Host>, Error> {
        util::parse_single_value(values)
    }

    fn to_values(&self, values: &mut ToValues) -> Result<(), Error> {
        let value = match self.port {
            None | Some(80) | Some(443) => HeaderValue::from_str(&self.hostname),
            Some(port) => HeaderValue::from_str(&format!("{}:{}", self.hostname, port)),
        };
        values.append(value.map_err(Error::new)?);
        Ok(())
    }
}

impl FromStr for Host {
    type Err = HostParseError;

    fn from_str(s: &str) -> Result<Host, HostParseError> {
        let (hostname, port) = match s.rfind(':') {
            Some(idx) => (
                &s[..idx],
                Some(s[idx + 1..].parse().map_err(|_| HostParseError(()))?),
            ),
            None => (s, None),
        };

        Ok(Host::new(hostname, port))
    }
}

#[derive(Debug)]
pub struct HostParseError(());

impl fmt::Display for HostParseError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("invalid host:port pair")
    }
}

impl error::Error for HostParseError {
    fn description(&self) -> &str {
        "invalid host:port pair"
    }
}
