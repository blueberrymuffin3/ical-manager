use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use maud::html;
use sqlx::SqlitePool;

use crate::presentation::error::{make_error_page, ServerResult};

#[axum::debug_handler]
pub async fn export(
    State(pool): State<SqlitePool>,
    Path(mut code): Path<String>,
) -> ServerResult<Response> {
    // Remove any extensions
    if let Some(dot) = code.find('.') {
        code.truncate(dot);
    }
    code.make_ascii_lowercase();

    let Some(feed) = sqlx::query!("SELECT * FROM Feed WHERE link_code = ?", code)
        .fetch_optional(&pool)
        .await?
    else {
        return Ok(make_error_page(
            "404 Not Found",
            html!((format_args!("No feed found with link code {code:?}"))),
            StatusCode::NOT_FOUND,
        ));
    };

    Ok(format!("Foud {feed:?}").into_response())
}
