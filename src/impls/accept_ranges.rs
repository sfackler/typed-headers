use http::header::ACCEPT_RANGES;

header! {
    (AcceptRanges, ACCEPT_RANGES) => (RangeUnit)+
}

token! {
    RangeUnit, InvalidRangeUnit => {
        BYTES => "bytes",
        NONE => "none",
    }
}
