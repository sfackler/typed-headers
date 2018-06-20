use http::header::ACCEPT;
use mime::Mime;

use QualityItem;

header! {
    /// `Accept` header, defined in [RFC7231](http://tools.ietf.org/html/rfc7231#section-5.3.2)
    ///
    /// The `Accept` header field can be used by user agents to specify
    /// response media types that are acceptable.  Accept header fields can
    /// be used to indicate that the request is specifically limited to a
    /// small set of desired types, as in the case of a request for an
    /// in-line image
    ///
    /// # ABNF
    ///
    /// ```text
    /// Accept = #( media-range [ accept-params ] )
    ///
    /// media-range    = ( "*/*"
    ///                  / ( type "/" "*" )
    ///                  / ( type "/" subtype )
    ///                  ) *( OWS ";" OWS parameter )
    /// accept-params  = weight *( accept-ext )
    /// accept-ext = OWS ";" OWS token [ "=" ( token / quoted-string ) ]
    /// ```
    ///
    /// # Example values
    /// * `audio/*; q=0.2, audio/basic`
    /// * `text/plain; q=0.5, text/html, text/x-dvi; q=0.8, text/x-c`
    (Accept, ACCEPT) => (QualityItem<Mime>)*
}

#[cfg(test)]
mod test {
    use {util, Quality, QualityItem};

    use super::*;

    #[test]
    fn rfc1() {
        util::test_round_trip(
            &Accept(vec![
                QualityItem::new("audio/*".parse().unwrap(), Quality::from_u16(200)),
                QualityItem::new("audio/basic".parse().unwrap(), Quality::from_u16(1000)),
            ]),
            &["audio/*; q=0.2, audio/basic"],
        );
    }

    #[test]
    fn rfc2() {
        util::test_round_trip(
            &Accept(vec![
                QualityItem::new("text/plain".parse().unwrap(), Quality::from_u16(500)),
                QualityItem::new("text/html".parse().unwrap(), Quality::from_u16(1000)),
                QualityItem::new("text/x-dvi".parse().unwrap(), Quality::from_u16(800)),
                QualityItem::new("text/x-c".parse().unwrap(), Quality::from_u16(1000)),
            ]),
            &["text/plain; q=0.5, text/html, text/x-dvi; q=0.8, text/x-c"],
        );
    }
}
