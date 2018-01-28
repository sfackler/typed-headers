use http::header::ACCEPT_ENCODING;

use {ContentCoding, QualityItem};

header! {
    (AcceptEncoding, ACCEPT_ENCODING) => (QualityItem<ContentCoding>)*
}
