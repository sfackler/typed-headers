use mime::Mime;
use http::header::ACCEPT;

use QualityItem;

header! {
    (Accept, ACCEPT) => (QualityItem<Mime>)*
}
