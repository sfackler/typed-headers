use http::header::ACCESS_CONTROL_MAX_AGE;

header! {
    /// `Access-Control-Max-Age` header, part of
    /// [CORS](http://www.w3.org/TR/cors/#access-control-max-age-response-header)
    ///
    /// The `Access-Control-Max-Age` header indicates how long the results of a
    /// preflight request can be cached in a preflight result cache.
    ///
    /// # ABNF
    ///
    /// ```text
    /// Access-Control-Max-Age = \"Access-Control-Max-Age\" \":\" delta-seconds
    /// ```
    ///
    /// # Example values
    ///
    /// * `531`
    (AccessControlMaxAge, ACCESS_CONTROL_MAX_AGE) => [u32]
}
