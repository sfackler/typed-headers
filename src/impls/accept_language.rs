use http::header::ACCEPT_LANGUAGE;

use super::QualityItem;
use language_tags::LanguageTag;

header! {
    /// `Accept-Language` header, defined in
    /// [RFC7231](https://tools.ietf.org/html/rfc7231#section-5.3.5)
    ///
    /// The "Accept-Language" header field can be used by user agents to
    /// indicate the set of natural languages that are preferred in the
    /// response.
    (AcceptLanguage, ACCEPT_LANGUAGE) => (QualityItem<LanguageTag>)*
}
