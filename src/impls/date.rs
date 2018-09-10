use HttpDate;
use header::DATE;

header! {
    /// `Date` header, defined in [RFC7231](http://tools.ietf.org/html/rfc7231#section-7.1.1.2)
    ///
    /// The `Date` header field represents the date and time at which the
    /// message was originated.
    ///
    /// # ABNF
    ///
    /// ```text
    /// Date = HTTP-date
    /// ```
    ///
    /// # Example values
    ///
    /// * `Tue, 15 Nov 1994 08:12:31 GMT`
    (Date, DATE) => [HttpDate]
}
