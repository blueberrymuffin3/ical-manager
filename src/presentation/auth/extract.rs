use std::{borrow::Cow, str::FromStr};

use anyhow::{anyhow, Context};
use async_trait::async_trait;
use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
    response::{IntoResponse, Response},
};
use http::{header::LOCATION, StatusCode, Uri};
use sqlx::SqlitePool;

use crate::{
    data::user::User,
    presentation::{
        cookies::ReadCookie,
        error::{make_error_page_auto, InternalServerError},
        htmx::is_htmx_request,
    },
};

use super::{
    logic::AuthClaim,
    pages::{LoginPageParams, LOCATION_LOGIN},
};

pub struct Authenticated(pub User);
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

fn make_unauthenticated_response(
    parts: &Parts,
) -> Result<UnauthenticatedResponse, InternalServerError> {
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

pub enum AuthenticatedRejection {
    UnauthenticatedResponse(UnauthenticatedResponse),
    InternalServerError(InternalServerError),
}
impl IntoResponse for AuthenticatedRejection {
    fn into_response(self) -> Response {
        match self {
            AuthenticatedRejection::UnauthenticatedResponse(res) => res.into_response(),
            AuthenticatedRejection::InternalServerError(res) => res.into_response(),
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Authenticated
where
    S: Send + Sync,
    ReadCookie<AuthClaim>: FromRequestParts<S>,
    SqlitePool: FromRef<S>,
{
    type Rejection = AuthenticatedRejection;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let auth_claim = ReadCookie::<AuthClaim>::from_request_parts(parts, state)
            .await
            .map_err(|err| {
                AuthenticatedRejection::InternalServerError(InternalServerError(
                    err.into_response(),
                ))
            })?
            .0;

        let pool = SqlitePool::from_ref(state);

        let user = match auth_claim {
            Some(claim) => User::select_by_id(claim.user_id, &pool)
                .await
                .map_err(|err| AuthenticatedRejection::InternalServerError(err.into()))?,
            None => None,
        };

        match user {
            Some(user) => Ok(Self(user)),
            None => match make_unauthenticated_response(&parts) {
                Ok(res) => Err(AuthenticatedRejection::UnauthenticatedResponse(res)),
                Err(err) => Err(AuthenticatedRejection::InternalServerError(err)),
            },
        }
    }
}
