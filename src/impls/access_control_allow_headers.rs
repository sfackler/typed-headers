use http::header::{HeaderName, ACCESS_CONTROL_ALLOW_HEADERS};
header! {
    /// `Access-Control-Allow-Headers` header, part of
    /// [CORS](http://www.w3.org/TR/cors/#access-control-allow-headers-response-header)
    ///
    /// The `Access-Control-Allow-Headers` header indicates, as part of the
    /// response to a preflight request, which header field names can be used
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
    (AccessControlAllowHeaders, ACCESS_CONTROL_ALLOW_HEADERS) => (HeaderName)*
}
