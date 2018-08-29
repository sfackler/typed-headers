use http::header::REFERER;

header! {
    /// `Referer` header, defined in
    /// [RFC7231](http://tools.ietf.org/html/rfc7231#section-5.5.2)
    ///
    /// The `Referer` [sic] header field allows the user agent to specify a
    /// URI reference for the resource from which the target URI was obtained
    /// (i.e., the "referrer", though the field name is misspelled).  A user
    /// agent MUST NOT include the fragment and userinfo components of the
    /// URI reference, if any, when generating the Referer field value.
    ///
    /// # ABNF
    ///
    /// ```text
    /// Referer = absolute-URI / partial-URI
    /// ```
    ///
    /// # Example values
    ///
    /// * `http://www.example.org/hypertext/Overview.html`
    // TODO Use URL
    (Referer, REFERER) => [String]
}

