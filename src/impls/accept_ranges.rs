use http::header::ACCEPT_RANGES;

use RangeUnit;

header! {
    (AcceptRanges, ACCEPT_RANGES) => (RangeUnit)+
}
