use http::header::HOST;
use http::uri::Authority;

header! {
    (Host, HOST) => [Authority]
}
