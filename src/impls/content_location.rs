use http::header::CONTENT_LOCATION;
header! {
    /// `Content-Location` header, defined in
    /// [RFC7231](https://tools.ietf.org/html/rfc7231#section-3.1.4.2)
    ///
    /// The header can be used by both the client in requests and the server
    /// in responses with different semantics. Client sets `Content-Location`
    /// to refer to the URI where original representation of the body was
    /// obtained.
    ///
    /// In responses `Content-Location` represents URI for the representation
    /// that was content negotiated, created or for the response payload.
    ///
    /// # ABNF
    ///
    /// ```text
    /// Content-Location = absolute-URI / partial-URI
    /// ```
    ///
    /// # Example values
    ///
    /// * `/hypertext/Overview.html`
    /// * `http://www.example.org/hypertext/Overview.html`
    // TODO: use URL
    (ContentLocation, CONTENT_LOCATION) => [String]
}
