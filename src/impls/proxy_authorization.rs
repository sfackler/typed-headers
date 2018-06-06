use http::header::PROXY_AUTHORIZATION;

use Credentials;

header! {
    /// `Proxy-Authorization` header, defined in [RFC7235](https://tools.ietf.org/html/rfc7235#section-4.4)
    ///
    /// The `Proxy-Authorization` header field allows a user agent to authenticate
    /// itself with an HTTP proxy -- usually, but not necessarily, after
    /// receiving a 407 (Proxy Authentication Required) response and the
    /// `Proxy-Authenticate` header. Its value consists of credentials containing
    /// the authentication information of the user agent for the realm of the
    /// resource being requested.
    ///
    /// # ABNF
    ///
    /// ```text
    /// Authorization = credentials
    /// ```
    ///
    /// # Example values
    /// * `Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ==`
    /// * `Bearer fpKL54jvWmEGVoRdCNjG`
    (ProxyAuthorization, PROXY_AUTHORIZATION) => [Credentials]
}

#[cfg(test)]
mod test {
    use super::*;
    use {util, Token68};

    #[test]
    fn rfc1() {
        util::test_round_trip(
            &ProxyAuthorization(Credentials::basic("Aladdin", "open sesame").unwrap()),
            &["Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ=="],
        );
    }

    #[test]
    fn rfc2() {
        util::test_round_trip(
            &ProxyAuthorization(Credentials::bearer(
                Token68::new("fpKL54jvWmEGVoRdCNjG").unwrap(),
            )),
            &["Bearer fpKL54jvWmEGVoRdCNjG"],
        );
    }
}
