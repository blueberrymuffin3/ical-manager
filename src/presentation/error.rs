use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use maud::{html, Markup};

use super::layout::base_layout;

pub type ServerResult<T> = Result<T, InternalServerError>;

pub struct InternalServerError(pub Response);

impl<T> From<T> for InternalServerError
where
    anyhow::Error: From<T>,
{
    fn from(value: T) -> Self {
        let error: anyhow::Error = value.into();

        let content = format_error_markup(error);

        Self(make_error_page_auto(
            content,
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}

pub fn format_error_markup(error: anyhow::Error) -> Markup {
    let content = html!(
        pre { code {
            (format_args!("{:?}", error))
        }}
    );
    content
}

impl IntoResponse for InternalServerError {
    fn into_response(self) -> axum::response::Response {
        self.0
    }
}

pub fn make_error_page_auto(content: Markup, status: StatusCode) -> Response {
    make_error_page(&status.to_string(), content, status)
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
    make_error_page_auto(
        html!(
            a href="/" { "Go Home" }
        ),
        StatusCode::NOT_FOUND,
    )
}
