use std::convert::Infallible;

use axum::{
    debug_handler,
    extract::{FromRef, Path, Query, State},
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use http::{header::LOCATION, StatusCode};
use maud::html;
use openidconnect::{AuthorizationCode, CsrfToken, Nonce};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::{
    presentation::{
        cookies::{AutoCookie, CookieType, GenerateNewCookie, ReadCookie, SetCookie},
        error::{make_error_page_auto, InternalServerError, ServerResult},
        layout::layout,
    },
    AppState,
};

use super::{
    logic::{AuthClaim, AuthProvider},
    AuthManager,
};

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
    GenerateNewCookie(nonce): GenerateNewCookie<Nonce>,
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

#[debug_handler(state = AppState)]
async fn callback(
    Path(provider): Path<AuthProvider>,
    Query(params): Query<CallbackParams>,
    ReadCookie(csrf): ReadCookie<CsrfToken>,
    ReadCookie(nonce): ReadCookie<Nonce>,
    ReadCookie(return_to): ReadCookie<ReturnToCookie>,
    set_auth_claim: SetCookie<AuthClaim>,
    State(auth_manager): State<AuthManager>,
    State(db): State<SqlitePool>,
) -> ServerResult<Response> {
    let (csrf, nonce) = match (csrf, nonce) {
        (Some(csrf), Some(nonce)) => (csrf, nonce),
        _ => {
            return Err(InternalServerError(make_error_page_auto(
                html!("Bad Cookies"),
                StatusCode::BAD_REQUEST,
            )))
        }
    };

    if params.state.secret() != csrf.secret() {
        return Err(InternalServerError(make_error_page_auto(
            html!("Invalid CSRF Token"),
            StatusCode::BAD_REQUEST,
        )));
    }

    let auth_claim = auth_manager
        .setup_user(provider, params.code, nonce, &db)
        .await?;

    set_auth_claim.set(&auth_claim);

    Ok((
        StatusCode::FOUND,
        [(
            LOCATION,
            return_to.as_ref().map(|x| x.0.as_str()).unwrap_or("/"),
        )],
    )
        .into_response())
}

pub fn router<State>() -> Router<State>
where
    State: Clone + Send + Sync + 'static,
    tower_cookies::Key: FromRef<State>,
    AuthManager: FromRef<State>,
    SqlitePool: FromRef<State>,
{
    Router::<State>::new()
        .route("/", get(login))
        .route("/:provider", post(do_login))
        .route("/:provider/callback", get(callback))
}
