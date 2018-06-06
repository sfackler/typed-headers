use http::header::CONTENT_TYPE;

use mime::Mime;

header! {
    /// `Content-Type` header, defined in
    /// [RFC7231](http://tools.ietf.org/html/rfc7231#section-3.1.1.5)
    ///
    /// The `Content-Type` header field indicates the media type of the
    /// associated representation: either the representation enclosed in the
    /// message payload or the selected representation, as determined by the
    /// message semantics.  The indicated media type defines both the data
    /// format and how that data is intended to be processed by a recipient,
    /// within the scope of the received message semantics, after any content
    /// codings indicated by Content-Encoding are decoded.
    ///
    /// # ABNF
    ///
    /// ```text
    /// Content-Type = media-type
    /// ```
    ///
    /// # Example values
    ///
    /// * `text/html; charset=utf-8`
    /// * `application/json`
    (ContentType, CONTENT_TYPE) => [Mime]
}
