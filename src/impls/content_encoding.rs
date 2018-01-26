use http::header::CONTENT_ENCODING;

use impls::encoding::Encoding;

header! {
    (ContentEncoding, CONTENT_ENCODING) => (Encoding)+
}
