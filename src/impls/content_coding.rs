token! {
    /// A content coding, used in the `Accept-Encoding` and `Content-Encoding` headers.
    ContentCoding => {
        /// The Brotli coding, as specified in [RFC7932].
        ///
        /// [RFC7932]: https://tools.ietf.org/html/rfc7932
        BROTLI => "br" => [],
        /// The Gzip coding, as specified in [RFC7230].
        ///
        /// [RFC7230]: https://tools.ietf.org/html/rfc7230#section-4.2.3
        GZIP => "gzip" => ["x-gzip"],
        /// The Deflate coding, as specified in [RFC7230].
        ///
        /// [RFC7230]: https://tools.ietf.org/html/rfc7230#section-4.2.2
        DEFLATE => "deflate" => [],
        /// The Compress coding, as specified in [RFC7230].
        ///
        /// [RFC7230]: https://tools.ietf.org/html/rfc7230#section-4.2.1
        COMPRESS => "compress" => ["x-compress"],
        /// The identity coding.
        IDENTITY => "identity" => [],
        /// A wildcard, used in the `Accept-Encoding` header to indicate that all codings are acceptable.
        STAR => "*" => [],
    }
}
