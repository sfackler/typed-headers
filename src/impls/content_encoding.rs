use http::header::CONTENT_ENCODING;

use ContentCoding;

header! {
    /// `Content-Encoding` header, defined in
    /// [RFC7231](http://tools.ietf.org/html/rfc7231#section-3.1.2.2)
    ///
    /// The `Content-Encoding` header field indicates what content codings
    /// have been applied to the representation, beyond those inherent in the
    /// media type, and thus what decoding mechanisms have to be applied in
    /// order to obtain data in the media type referenced by the Content-Type
    /// header field.  Content-Encoding is primarily used to allow a
    /// representation's data to be compressed without losing the identity of
    /// its underlying media type.
    ///
    /// # ABNF
    ///
    /// ```text
    /// Content-Encoding = 1#content-coding
    /// ```
    ///
    /// # Example values
    ///
    /// * `gzip`
    (ContentEncoding, CONTENT_ENCODING) => (ContentCoding)+
}
