use std::convert::Infallible;

use axum::{
    extract::{FromRef, Path, Query, State},
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use http::StatusCode;
use maud::html;
use openidconnect::{AuthorizationCode, CsrfToken, Nonce};
use serde::{Deserialize, Serialize};

use crate::presentation::{
    cookies::{AutoCookie, CookieType, DeleteCookie, NewCookie, ReadCookie, SetCookie},
    error::{make_error_page_auto, ServerResult},
    layout::layout,
};

use super::{logic::AuthProvider, AuthManager};

pub const LOCATION_LOGIN: &str = "/login";

#[derive(Serialize, Deserialize)]
pub struct LoginPageParams {
    pub return_to: Option<String>,
}

struct ReturnToCookie(String);
impl CookieType for ReturnToCookie {
    fn name() -> &'static str {
        "return-to"
    }

    fn generate() -> Self {
        Self("/".to_owned())
    }

    type Rejection = Infallible;

    fn encode(&self) -> String {
        self.0.to_owned()
    }

    fn decode(value: &str) -> Result<Self, Self::Rejection> {
        Ok(Self(value.to_owned()))
    }
}

async fn login(Query(params): Query<LoginPageParams>) -> ServerResult<Response> {
    let params = serde_urlencoded::to_string(params)?;
    let make_target = move |name: &str, params: &str| format!("{LOCATION_LOGIN}/{name}?{params}");

    Ok(layout(html!(
        h2 { "Login" }
        button hx-post=(make_target("github", &params)) {"GitHub"}
        button hx-post=(make_target("google", &params)) {"Google"}
    ))
    .into_response())
}

async fn do_login(
    Path(provider): Path<AuthProvider>,
    Query(params): Query<LoginPageParams>,
    AutoCookie(csrf): AutoCookie<CsrfToken>,
    NewCookie(nonce): NewCookie<Nonce>,
    set_return_to: SetCookie<ReturnToCookie>,
    State(manager): State<AuthManager>,
) -> ServerResult<impl IntoResponse> {
    match params.return_to {
        Some(return_to) => set_return_to.set(&ReturnToCookie(return_to)),
        None => set_return_to.delete(),
    };

    let url = manager.make_url(provider, csrf, nonce).await;

    Ok([("HX-Redirect", url)].into_response())
}

#[derive(Deserialize)]
struct CallbackParams {
    code: AuthorizationCode,
    state: CsrfToken,
}

async fn callback(
    Query(params): Query<CallbackParams>,
    ReadCookie(csrf): ReadCookie<CsrfToken>,
    ReadCookie(nonce): ReadCookie<Nonce>,
    State(auth_manager): State<AuthManager>,
) -> impl IntoResponse {
    let (csrf, nonce) = match (csrf, nonce) {
        (Some(csrf), Some(nonce)) => (csrf, nonce),
        _ => return make_error_page_auto(html!("Bad Cookies"), StatusCode::BAD_REQUEST),
    };

    if params.state.secret() != csrf.secret() {
        return make_error_page_auto(html!("Invalid CSRF Token"), StatusCode::BAD_GATEWAY);
    }

    "Hello World".into_response()
}

pub fn router<State>() -> Router<State>
where
    State: Clone + Send + Sync + 'static,
    tower_cookies::Key: FromRef<State>,
    AuthManager: FromRef<State>,
{
    Router::<State>::new()
        .route("/", get(login))
        .route("/callback", get(callback))
        .route("/:provider", post(do_login))
}
