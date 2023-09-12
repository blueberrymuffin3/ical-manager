use std::{borrow::Cow, str::FromStr};

use anyhow::{anyhow, Context};
use async_trait::async_trait;
use axum::{
    extract::FromRequestParts,
    http::request::Parts,
    response::{IntoResponse, Response},
};
use http::{header::LOCATION, StatusCode, Uri};
use tower_cookies::Cookies;

use crate::presentation::{
    error::{make_error_page_auto, InternalServerError},
    htmx::is_htmx_request,
};

use super::pages::{LOCATION_LOGIN, LoginPageParams};

pub struct Authenticated;
pub struct UnauthenticatedResponse {
    return_to: String,
    is_htmx: bool,
}

impl UnauthenticatedResponse {
    fn into_redirect_location(self) -> Result<String, serde_urlencoded::ser::Error> {
        Ok(format!(
            "{LOCATION_LOGIN}?{}",
            serde_urlencoded::to_string(LoginPageParams {
                return_to: Some(self.return_to)
            })?
        ))
    }
}

impl IntoResponse for UnauthenticatedResponse {
    fn into_response(self) -> Response {
        let is_htmx = self.is_htmx;
        let location = match self.into_redirect_location() {
            Ok(location) => location,
            Err(err) => {
                return make_error_page_auto(
                    format_args!("{:?}", anyhow!(err)),
                    StatusCode::BAD_REQUEST,
                )
            }
        };

        if is_htmx {
            ([("HX-Redirect", location)], StatusCode::OK).into_response()
        } else {
            ([(LOCATION, location)], StatusCode::SEE_OTHER).into_response()
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for UnauthenticatedResponse {
    /// If the extractor fails it'll use this "rejection" type. A rejection is
    /// a kind of error that can be converted into a response.
    type Rejection = InternalServerError;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let is_htmx = is_htmx_request(&parts.headers);

        let return_to = if is_htmx {
            let uri = parts
                .headers
                .get("HX-Current-URL")
                .context("HX-Current-URL header missing")?
                .to_str()
                .context("Invalid HX-Current-URL")?;
            let url = Uri::from_str(uri).context("HX-Current-URL")?;
            Cow::Owned(url)
        } else {
            Cow::Borrowed(&parts.uri)
        };

        let return_to = return_to
            .as_ref()
            .path_and_query()
            .context("Invalid return url")?;

        Ok(UnauthenticatedResponse {
            is_htmx,
            return_to: return_to.to_string(),
        })
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Authenticated
where
    S:,
{
    type Rejection = Result<UnauthenticatedResponse, InternalServerError>;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let cookies = Cookies::from_request_parts(parts, &())
            .await
            .map_err(|x| Err(InternalServerError(x.into_response())))?;

        log::info!("Cookies are {cookies:?}");

        // Ok(Authenticated)
        return Err(UnauthenticatedResponse::from_request_parts(parts, &()).await);
    }
}
