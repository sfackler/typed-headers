use http::header::{HeaderName, VARY};
header! {
    /// `Vary` header, defined in [RFC7231](https://tools.ietf.org/html/rfc7231#section-7.1.4)
    ///
    /// The "Vary" header field in a response describes what parts of a
    /// request message, aside from the method, Host header field, and
    /// request target, might influence the origin server's process for
    /// selecting and representing this response.  The value consists of
    /// either a single asterisk ("*") or a list of header field names
    /// (case-insensitive).
    ///
    /// # ABNF
    ///
    /// ```text
    /// Vary = "*" / 1#field-name
    /// ```
    ///
    /// # Example values
    ///
    /// * `accept-encoding, accept-language`
    (Vary, VARY) => (Any / (HeaderName)+)
}
