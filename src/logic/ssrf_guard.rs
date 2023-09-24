use futures::FutureExt;
use hyper::{client::connect::dns::GaiResolver, service::Service};
use ip_rfc::{global, global_v4, global_v6};
use reqwest::dns::Resolve;
use url::Url;

#[derive(Debug, thiserror::Error)]
#[error("Attempted to connected to an invalid ip")]
pub struct InvalidIpError;
#[derive(Debug, thiserror::Error)]
#[error("Too many redirects")]
struct TooManyRedirects;

pub fn check_url(url: &Url) -> Result<(), InvalidIpError> {
    let host = url.host();
    let ok = match host {
        None => true,
        Some(url::Host::Domain(..)) => true,
        Some(url::Host::Ipv4(addr)) => global_v4(&addr),
        Some(url::Host::Ipv6(addr)) => global_v6(&addr),
    };

    if ok {
        Ok(())
    } else {
        Err(InvalidIpError)
    }
}

pub struct SSRFSafeResolver;

impl Resolve for SSRFSafeResolver {
    fn resolve(&self, name: hyper::client::connect::dns::Name) -> reqwest::dns::Resolving {
        Box::pin(GaiResolver::new().call(name).map(|result| {
            result
                .map(|addrs| -> reqwest::dns::Addrs { Box::new(addrs.filter(|x| global(&x.ip()))) })
                .map_err(|err| -> tower_http::BoxError { Box::new(err) })
        }))
    }
}

const MAX_REDIRECTS: usize = 10;

pub fn ssrf_safe_redirect_policy() -> reqwest::redirect::Policy {
    reqwest::redirect::Policy::custom(|attempt| {
        if attempt.previous().len() >= MAX_REDIRECTS {
            return attempt.error(TooManyRedirects);
        }

        if let Err(err) = check_url(attempt.url()) {
            return attempt.error(err);
        }

        attempt.follow()
    })
}
