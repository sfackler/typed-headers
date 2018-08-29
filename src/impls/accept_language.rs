use language_tags::LanguageTag;
use QualityItem;
use http::header::ACCEPT_LANGUAGE;

header! {
    /// `Accept-Language` header, defined in
    /// [RFC7231](http://tools.ietf.org/html/rfc7231#section-5.3.5)
    ///
    /// The `Accept-Language` header field can be used by user agents to
    /// indicate the set of natural languages that are preferred in the
    /// response.
    ///
    /// # ABNF
    ///
    /// ```text
    /// Accept-Language = 1#( language-range [ weight ] )
    /// language-range  = <language-range, see [RFC4647], Section 2.1>
    /// ```
    ///
    /// # Example values
    /// * `da, en-gb;q=0.8, en;q=0.7`
    /// * `en-us;q=1.0, en;q=0.5, fr`
    (AcceptLanguage, ACCEPT_LANGUAGE) => (QualityItem<LanguageTag>)+
}
