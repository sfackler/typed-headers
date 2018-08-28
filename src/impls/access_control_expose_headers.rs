use http::header::ACCESS_CONTROL_EXPOSE_HEADERS;
use http::header::HeaderName;

header! {
    /// `Access-Control-Expose-Headers` header, part of
    /// [CORS](http://www.w3.org/TR/cors/#access-control-expose-headers-response-header)
    ///
    /// The Access-Control-Expose-Headers header indicates which headers are safe to expose to the
    /// API of a CORS API specification.
    ///
    /// # ABNF
    ///
    /// ```text
    /// Access-Control-Expose-Headers = "Access-Control-Expose-Headers" ":" #field-name
    /// ```
    ///
    /// # Example values
    /// * `ETag, Content-Length`
    ///
    (AccessControlExposeHeaders, ACCESS_CONTROL_EXPOSE_HEADERS) => (HeaderName)*
}
