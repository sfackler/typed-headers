use http::header::IF_MODIFIED_SINCE;
use HttpDate;

header! {
    /// `If-Modified-Since` header, defined in
    /// [RFC7232](http://tools.ietf.org/html/rfc7232#section-3.3)
    ///
    /// The `If-Modified-Since` header field makes a GET or HEAD request
    /// method conditional on the selected representation's modification date
    /// being more recent than the date provided in the field-value.
    /// Transfer of the selected representation's data is avoided if that
    /// data has not changed.
    ///
    /// # ABNF
    ///
    /// ```text
    /// If-Unmodified-Since = HTTP-date
    /// ```
    ///
    /// # Example values
    /// * `Sat, 29 Oct 1994 19:43:31 GMT`
    (IfModifiedSince, IF_MODIFIED_SINCE) => [HttpDate]
}

