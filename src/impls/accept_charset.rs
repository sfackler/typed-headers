use http::header::ACCEPT_CHARSET;

use {Charset, QualityItem};

header! {
    (AcceptCharset, ACCEPT_CHARSET) => (QualityItem<Charset>)+
}

#[cfg(test)]
mod test {
    use util;
    use Quality;

    use super::*;

    #[test]
    fn rfc() {
        util::test_round_trip(
            &AcceptCharset(vec![
                QualityItem::new(Charset::ISO_8859_5, Quality::from_u16(1000)),
                QualityItem::new(Charset::new("unicode-1-1").unwrap(), Quality::from_u16(800)),
            ]),
            &["iso-8859-5, unicode-1-1; q=0.8"],
        );
    }
}
