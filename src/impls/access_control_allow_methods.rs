use http::Method;
use http::header::ACCESS_CONTROL_ALLOW_METHODS;

header! {
    /// `Access-Control-Allow-Methods` header, part of
    /// [CORS](http://www.w3.org/TR/cors/#access-control-allow-methods-response-header)
    ///
    /// The `Access-Control-Allow-Methods` header indicates, as part of the
    /// response to a preflight request, which methods can be used during the
    /// actual request.
    ///
    /// # ABNF
    ///
    /// ```text
    /// Access-Control-Allow-Methods: "Access-Control-Allow-Methods" ":" #Method
    /// ```
    ///
    /// # Example values
    /// * `PUT, DELETE, XMODIFY`
    (AccessControlAllowMethods, ACCESS_CONTROL_ALLOW_METHODS) => (Method)*
}
