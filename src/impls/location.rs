use http::header::LOCATION;

header! {
    /// `Location` header, defined in
    /// [RFC7231](http://tools.ietf.org/html/rfc7231#section-7.1.2)
    ///
    /// The `Location` header field is used in some responses to refer to a
    /// specific resource in relation to the response.  The type of
    /// relationship is defined by the combination of request method and
    /// status code semantics.
    ///
    /// # ABNF
    ///
    /// ```text
    /// Location = URI-reference
    /// ```
    ///
    /// # Example values
    /// * `/People.html#tim`
    /// * `http://www.example.net/index.html`
    // TODO: Use URL
    (Location, LOCATION) => [String]
}

