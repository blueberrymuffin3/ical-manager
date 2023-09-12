use std::convert::Infallible;

use async_trait::async_trait;
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, HeaderMap, HeaderValue},
};

pub fn is_htmx_request(headers: &HeaderMap) -> bool {
    headers.get("HX-Request") == Some(&HeaderValue::from_static("true"))
}

pub struct HxWrap {
    is_htmx: bool,
}

impl HxWrap {
    pub fn wrap<T>(&self, inner: T, wrap: impl FnOnce(T) -> T) -> T {
        if self.is_htmx {
            inner
        } else {
            wrap(inner)
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for HxWrap {
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        Ok(Self {
            is_htmx: is_htmx_request(&parts.headers),
        })
    }
}
