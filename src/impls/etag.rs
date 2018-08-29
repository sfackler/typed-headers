use EntityTag;
use http::header::ETAG;

header! {
    /// `ETag` header, defined in [RFC7232](http://tools.ietf.org/html/rfc7232#section-2.3)
    ///
    /// The `ETag` header field in a response provides the current entity-tag
    /// for the selected representation, as determined at the conclusion of
    /// handling the request.  An entity-tag is an opaque validator for
    /// differentiating between multiple representations of the same
    /// resource, regardless of whether those multiple representations are
    /// due to resource state changes over time, content negotiation
    /// resulting in multiple representations being valid at the same time,
    /// or both.  An entity-tag consists of an opaque quoted string, possibly
    /// prefixed by a weakness indicator.
    ///
    /// # ABNF
    ///
    /// ```text
    /// ETag       = entity-tag
    /// ```
    ///
    /// # Example values
    ///
    /// * `"xyzzy"`
    /// * `W/"xyzzy"`
    /// * `""`
    (ETag, ETAG) => [EntityTag]
}

