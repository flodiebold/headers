use super::authorization::{Authorization, Credentials};

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
/// Proxy-Authorization = credentials
/// ```
///
/// # Example values
/// * `Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ==`
/// * `Bearer fpKL54jvWmEGVoRdCNjG`
///
/// # Examples
///
#[derive(Clone, PartialEq, Debug)]
pub struct ProxyAuthorization<C: Credentials>(pub C);

impl<C: Credentials> ::Header for ProxyAuthorization<C> {
    const NAME: &'static ::HeaderName = &::http::header::PROXY_AUTHORIZATION;

    fn decode(values: &mut ::Values) -> Option<Self> {
        Authorization::decode(values)
            .map(|auth| ProxyAuthorization(auth.0))
    }

    fn encode(&self, values: &mut ::ToValues) {
        let value = self.0.encode();
        debug_assert!(
            value.as_bytes().starts_with(C::SCHEME.as_bytes()),
            "Credentials::encode should include its scheme: scheme = {:?}, encoded = {:?}",
            C::SCHEME,
            value,
        );

        values.append(value);
    }
}

