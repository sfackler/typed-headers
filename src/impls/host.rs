use http::header::{self, HeaderName, HeaderValue, InvalidHeaderValue, HOST};
use std::str::FromStr;

use {parsing, Header, ParseError, ToValues};

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

    fn parse(values: &mut header::ValueIter<HeaderValue>) -> Result<Option<Host>, ParseError> {
        parsing::from_one_str(values)
    }

    fn to_values(&self, values: &mut ToValues) -> Result<(), InvalidHeaderValue> {
        let value = match self.port {
            None | Some(80) | Some(443) => HeaderValue::from_str(&self.hostname)?,
            Some(port) => HeaderValue::from_str(&format!("{}:{}", self.hostname, port))?,
        };
        values.append(value);
        Ok(())
    }
}

impl FromStr for Host {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Host, ParseError> {
        let (hostname, port) = match s.rfind(':') {
            Some(idx) => (
                &s[..idx],
                Some(s[idx + 1..].parse().map_err(ParseError::new)?),
            ),
            None => (s, None),
        };

        Ok(Host::new(hostname, port))
    }
}
