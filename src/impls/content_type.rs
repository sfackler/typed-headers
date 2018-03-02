use http::header::CONTENT_TYPE;

use mime::Mime;

header! {
    (ContentType, CONTENT_TYPE) => [Mime]
}
