use http::header::ALLOW;
use http::Method;

header! {
    (Allow, ALLOW) => (Method)*
}
