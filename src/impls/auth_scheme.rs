token! {
    /// An authorization scheme.
    AuthScheme => {
        /// Basic authentication, as defined in [RFC7617].
        ///
        /// [RFC7617]: https://tools.ietf.org/html/rfc7617
        BASIC => "Basic" => [],
        /// Bearer authentication, as defined in [RFC6750].
        ///
        /// [RFC6750]: https://tools.ietf.org/html/rfc6750
        BEARER => "Bearer" => [],
    }
}
