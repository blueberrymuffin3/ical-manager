use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use maud::{html, Markup};

use super::layout::base_layout;

pub type ServerResult<T> = Result<T, InternalServerError>;

pub struct InternalServerError(pub anyhow::Error);

impl<T> From<T> for InternalServerError
where
    anyhow::Error: From<T>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl IntoResponse for InternalServerError {
    fn into_response(self) -> axum::response::Response {
        let content = html!(
            pre { code {
                (format_args!("{:?}", self.0))
            }}
        );

        make_error_page(
            "500 Internal Server Error",
            content,
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    }
}

pub fn make_error_page(title: &str, content: Markup, status: StatusCode) -> Response {
    let html = base_layout(html!(
        div."medium-container" {
            h1 {(title)}
            (content)
        }
    ));

    (status, html).into_response()
}

pub fn make_404() -> Response {
    make_error_page(
        "404 Not Found",
        html!(
            a href="/" { "Go Home" }
        ),
        StatusCode::NOT_FOUND,
    )
}
