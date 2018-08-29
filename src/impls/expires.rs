use HttpDate;
use http::header::EXPIRES;

header! {
    /// `Expires` header, defined in [RFC7234](http://tools.ietf.org/html/rfc7234#section-5.3)
    ///
    /// The `Expires` header field gives the date/time after which the
    /// response is considered stale.
    ///
    /// The presence of an Expires field does not imply that the original
    /// resource will change or cease to exist at, before, or after that
    /// time.
    ///
    /// # ABNF
    ///
    /// ```text
    /// Expires = HTTP-date
    /// ```
    ///
    /// # Example values
    /// * `Thu, 01 Dec 1994 16:00:00 GMT`
    (Expires, EXPIRES) => [HttpDate]
}

