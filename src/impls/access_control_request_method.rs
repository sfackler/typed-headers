use http::Method;
use http::header::ACCESS_CONTROL_REQUEST_METHOD;

header! {
    /// `Access-Control-Request-Method` header, part of
    /// [CORS](http://www.w3.org/TR/cors/#access-control-request-method-request-header)
    ///
    /// The `Access-Control-Request-Method` header indicates which method will be
    /// used in the actual request as part of the preflight request.
    /// # ABNF
    ///
    /// ```text
    /// Access-Control-Request-Method: \"Access-Control-Request-Method\" \":\" Method
    /// ```
    ///
    /// # Example values
    /// * `GET`
    (AccessControlRequestMethod, ACCESS_CONTROL_REQUEST_METHOD) => [Method]
}
