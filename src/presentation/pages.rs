use std::time::Duration;

use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
};
use icondata::LuIcon;
use maud::{html, Markup};
use sqlx::SqlitePool;

use super::{
    error::{make_404, ServerResult},
    fragments::{self, feed_form, FeedType, FormValues},
    htmx::HxWrap,
    icon::icon_alt,
    layout::layout,
};

#[axum::debug_handler]
pub async fn feed_status(
    State(pool): State<SqlitePool>,
    Path((id,)): Path<(i64,)>,
) -> ServerResult<impl IntoResponse> {
    Ok(fragments::feed_status(&pool, id).await?.into_response())
}

#[axum::debug_handler]
pub async fn root(State(pool): State<SqlitePool>, hx_wrap: HxWrap) -> ServerResult<Markup> {
    let table = fragments::feed_table(&pool).await?;

    Ok(hx_wrap.wrap(table, |inner| {
        layout(html!(
            h2 {
                "Feeds "
                button."small-button"."muted-button"."round-button"
                    id="feed-spinner"
                    hx-get="/"
                    hx-target="#feeds-table"
                    hx-indicator
                {
                    div."htmx-spinner" {
                        (icon_alt(LuIcon::LuRefreshCw, "Refresh Feeds"))
                    }
                }
            }
            (inner)
        ))
    }))
}

#[axum::debug_handler]
pub async fn feed_edit(
    State(pool): State<SqlitePool>,
    Path((id,)): Path<(i64,)>,
) -> ServerResult<Response> {
    let Some(feed) = sqlx::query!(
        "SELECT name, source_link, ttl_seconds FROM Feed WHERE id = ?",
        id
    )
    .fetch_optional(&pool)
    .await?
    else {
        return Ok(make_404());
    };

    let values = FormValues {
        name: feed.name,
        feed_type: match feed.source_link {
            Some(_) => FeedType::Link,
            None => FeedType::Upload,
        },
        link: feed.source_link.unwrap_or_else(String::new),
        ttl: feed
            .ttl_seconds
            .map(|seconds| Duration::from_secs(seconds as u64)),
    };

    Ok(layout(html!(
        h2 { "Edit Feed" }
        (feed_form(Some(&values)))
    ))
    .into_response())
}
