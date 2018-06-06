use http::header::AUTHORIZATION;

use Credentials;

header! {
    /// `Authorization` header, defined in [RFC7235](https://tools.ietf.org/html/rfc7235#section-4.2)
    ///
    /// The `Authorization` header field allows a user agent to authenticate
    /// itself with an origin server -- usually, but not necessarily, after
    /// receiving a 401 (Unauthorized) response.  Its value consists of
    /// credentials containing the authentication information of the user
    /// agent for the realm of the resource being requested.
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
    (Authorization, AUTHORIZATION) => [Credentials]
}

#[cfg(test)]
mod test {
    use super::*;
    use {util, Token68};

    #[test]
    fn rfc1() {
        util::test_round_trip(
            &Authorization(Credentials::basic("Aladdin", "open sesame").unwrap()),
            &["Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ=="],
        );
    }

    #[test]
    fn rfc2() {
        util::test_round_trip(
            &Authorization(Credentials::bearer(
                Token68::new("fpKL54jvWmEGVoRdCNjG").unwrap(),
            )),
            &["Bearer fpKL54jvWmEGVoRdCNjG"],
        );
    }
}
