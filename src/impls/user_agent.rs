use http::header::USER_AGENT;

header! {
    /// `User-Agent` header, defined in
    /// [RFC7231](http://tools.ietf.org/html/rfc7231#section-5.5.3)
    ///
    /// The `User-Agent` header field contains information about the user
    /// agent originating the request, which is often used by servers to help
    /// identify the scope of reported interoperability problems, to work
    /// around or tailor responses to avoid particular user agent
    /// limitations, and for analytics regarding browser or operating system
    /// use.  A user agent SHOULD send a User-Agent field in each request
    /// unless specifically configured not to do so.
    ///
    /// # ABNF
    ///
    /// ```text
    /// User-Agent = product *( RWS ( product / comment ) )
    /// product         = token ["/" product-version]
    /// product-version = token
    /// ```
    ///
    /// # Example values
    ///
    /// * `CERN-LineMode/2.15 libwww/2.17b3`
    /// * `Bunnies`
    ///
    /// # Notes
    ///
    /// * The parser does not split the value
    (UserAgent, USER_AGENT) => [String]
}
