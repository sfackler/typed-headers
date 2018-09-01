use std::fmt;
use http::header::{self, HeaderName, HeaderValue, REFERRER_POLICY};
use {util, Error, Header, ToValues};

/// `Referrer-Policy` header, part of
/// [Referrer Policy](https://www.w3.org/TR/referrer-policy/#referrer-policy-header)
///
/// The `Referrer-Policy` HTTP header specifies the referrer
/// policy that the user agent applies when determining what
/// referrer information should be included with requests made,
/// and with browsing contexts created from the context of the
/// protected resource.
///
/// # ABNF
///
/// ```text
/// Referrer-Policy: 1#policy-token
/// policy-token   = "no-referrer" / "no-referrer-when-downgrade"
///                  / "same-origin" / "origin"
///                  / "origin-when-cross-origin" / "unsafe-url"
/// ```
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ReferrerPolicy {
    /// `no-referrer`
    NoReferrer,
    /// `no-referrer-when-downgrade`
    NoReferrerWhenDowngrade,
    /// `same-origin`
    SameOrigin,
    /// `origin`
    Origin,
    /// `origin-when-cross-origin`
    OriginWhenCrossOrigin,
    /// `unsafe-url`
    UnsafeUrl,
     /// `strict-origin`
    StrictOrigin,
    ///`strict-origin-when-cross-origin`
    StrictOriginWhenCrossOrigin,
}

impl Header for ReferrerPolicy {
    fn name() -> &'static HeaderName {
        &REFERRER_POLICY
    }

    #[inline]
    fn from_values(
        values: &mut header::ValueIter<HeaderValue>,
    ) -> Result<Option<ReferrerPolicy>, Error> {
        use self::ReferrerPolicy::*;
        // See https://www.w3.org/TR/referrer-policy/#determine-policy-for-token
        let headers: Option<Vec<String>> = util::parse_comma_delimited(values)?;

        for h in headers.unwrap_or(vec![]).iter().rev() {
            let slice = &h.to_ascii_lowercase()[..];
            match slice {
                "no-referrer" | "never" => return Ok(Some(NoReferrer)),
                "no-referrer-when-downgrade" | "default" => return Ok(Some(NoReferrerWhenDowngrade)),
                "same-origin" => return Ok(Some(SameOrigin)),
                "origin" => return Ok(Some(Origin)),
                "origin-when-cross-origin" => return Ok(Some(OriginWhenCrossOrigin)),
                "strict-origin" => return Ok(Some(StrictOrigin)),
                "strict-origin-when-cross-origin" => return Ok(Some(StrictOriginWhenCrossOrigin)),
                "unsafe-url" | "always" => return Ok(Some(UnsafeUrl)),
                _ => continue,
            }
        }

        Err(Error::invalid_value())
    }

    #[inline]
    fn to_values(&self, values: &mut ToValues) {
        util::encode_single_value(&self.to_string(), values);
    }
}

impl fmt::Display for ReferrerPolicy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ReferrerPolicy::*;
        f.write_str(match *self {
            NoReferrer => "no-referrer",
            NoReferrerWhenDowngrade => "no-referrer-when-downgrade",
            SameOrigin => "same-origin",
            Origin => "origin",
            OriginWhenCrossOrigin => "origin-when-cross-origin",
            StrictOrigin => "strict-origin",
            StrictOriginWhenCrossOrigin => "strict-origin-when-cross-origin",
            UnsafeUrl => "unsafe-url",
        })
    }
}
