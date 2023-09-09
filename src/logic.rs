use axum::{
    extract::{Path, State},
    http::{StatusCode},
    response::{IntoResponse, Response},
};
use maud::html;
use reqwest::header::{IntoHeaderName, CONTENT_TYPE};
use sqlx::SqlitePool;

use crate::{
    data::{feed::Feed, source::SourceTrait},
    presentation::error::{
        format_error_markup, make_error_page, make_error_page_auto, InternalServerError,
        ServerResult,
    },
};

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

    let Some(feed) = Feed::select_by_link_code(&code, &mut pool.begin().await?).await? else {
        return Ok(make_error_page_auto(
            html!((format_args!("No feed found with link code {code:?}"))),
            StatusCode::NOT_FOUND,
        ));
    };

    let data = feed.data.source.fetch().await.map_err(|error| {
        InternalServerError(make_error_page(
            "Error fetching feed contents",
            format_error_markup(error),
            StatusCode::BAD_GATEWAY,
        ))
    })?;

    // Ok(([(CONTENT_TYPE, "text/calendar")], data).into_response())
    Ok(([(CONTENT_TYPE, "text/plain")], data).into_response())
}
