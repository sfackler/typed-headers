use http::header::{HeaderName, ACCESS_CONTROL_REQUEST_HEADERS};

header! {
    /// `Access-Control-Request-Headers` header, part of
    /// [CORS](http://www.w3.org/TR/cors/#access-control-request-headers-request-header)
    ///
    /// The `Access-Control-Request-Headers` header indicates which headers will
    /// be used in the actual request as part of the preflight request.
    /// during the actual request.
    ///
    /// # ABNF
    ///
    /// ```text
    /// Access-Control-Allow-Headers: "Access-Control-Allow-Headers" ":" #field-name
    /// ```
    ///
    /// # Example values
    /// * `accept-language, date`
    (AccessControlRequestHeaders, ACCESS_CONTROL_REQUEST_HEADERS) => (HeaderName)*
}
