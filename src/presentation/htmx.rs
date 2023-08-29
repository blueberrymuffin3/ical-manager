use std::convert::Infallible;

use async_trait::async_trait;
use axum::{
    extract::FromRequest,
    http::{HeaderMap, HeaderValue, Request},
};

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
impl<S, B> FromRequest<S, B> for HxWrap
where
    // these bounds are required by `async_trait`
    B: Send + 'static,
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let headers = HeaderMap::from_request(req, state).await?;

        let is_htmx = headers.get("HX-Request") == Some(&HeaderValue::from_static("true"));
        Ok(Self { is_htmx })
    }
}
