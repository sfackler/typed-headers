use http::header::ACCEPT_ENCODING;

use {ContentCoding, QualityItem};

header! {
    /// `Accept-Encoding` header, defined in
    /// [RFC7231](http://tools.ietf.org/html/rfc7231#section-5.3.4)
    ///
    /// The `Accept-Encoding` header field can be used by user agents to
    /// indicate what response content-codings are
    /// acceptable in the response.  An  `identity` token is used as a synonym
    /// for "no encoding" in order to communicate when no encoding is
    /// preferred.
    ///
    /// # ABNF
    ///
    /// ```text
    /// Accept-Encoding  = #( codings [ weight ] )
    /// codings          = content-coding / "identity" / "*"
    /// ```
    ///
    /// # Example values
    /// * `compress, gzip`
    /// * ``
    /// * `*`
    /// * `compress;q=0.5, gzip;q=1`
    /// * `gzip;q=1.0, identity; q=0.5, *;q=0`
    (AcceptEncoding, ACCEPT_ENCODING) => (QualityItem<ContentCoding>)*
}
