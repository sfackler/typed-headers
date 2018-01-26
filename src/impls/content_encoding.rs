use http::header::CONTENT_ENCODING;

use ContentCoding;

header! {
    (ContentEncoding, CONTENT_ENCODING) => (ContentCoding)+
}
