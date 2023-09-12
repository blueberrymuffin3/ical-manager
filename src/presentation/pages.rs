use anyhow::Context;
use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
};
use axum_typed_multipart::TypedMultipart;
use icondata::LuIcon;
use maud::{html, Markup};
use sqlx::SqlitePool;

use crate::data::feed::Feed;

use super::{
    auth::Authenticated,
    error::{make_404, ServerResult},
    form::{feed_form, FeedFormValues, ValidationErrors},
    fragments,
    htmx::HxWrap,
    icon::{icon, icon_alt},
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
    let feeds = Feed::select(&mut pool.begin().await?)
        .await
        .context("Error fetching feed list")?;

    let table = fragments::feed_table(&feeds);

    Ok(hx_wrap.wrap(table, |inner| {
        layout(html!(
            h2 {
                "Feeds "

                button."small-button"."muted-button"."round-button"
                    id="feed-spinner"
                    hx-get="/"
                    hx-target="#feeds-table"
                    hx-swap="outerHTML"
                    hx-indicator
                {
                    div."htmx-spinner" {
                        (icon_alt(LuIcon::LuRefreshCw, "Refresh Feeds"))
                    }
                }
            }
            (inner)

            a."button" href="/feed/create" {
                (icon(LuIcon::LuPlus))
                " New"
            }

            p {
                "Note: feeds are only updated when requested to by your calendar app"
            }
        ))
    }))
}

#[axum::debug_handler]
pub async fn feed_edit_get(
    State(pool): State<SqlitePool>,
    Path((id,)): Path<(i64,)>,
) -> ServerResult<Response> {
    let Some(feed) = Feed::select_by_id(id, &mut pool.begin().await?).await? else {
        return Ok(make_404());
    };

    let values: FeedFormValues = feed.data.into();

    Ok(layout(html!(
        h2 { "Edit Feed" }
        (feed_form(Some(&values), ValidationErrors::default()))
    ))
    .into_response())
}

#[axum::debug_handler]
pub async fn feed_edit_post(
    State(pool): State<SqlitePool>,
    Path((id,)): Path<(i64,)>,
    TypedMultipart(form_values): TypedMultipart<FeedFormValues>,
) -> ServerResult<Response> {
    let mut txn = pool.begin().await?;

    let Some(mut feed) = Feed::select_by_id(id, &mut txn).await? else {
        return Ok(make_404());
    };

    match form_values.try_into_feed_data() {
        Ok(feed_data) => {
            feed.data = feed_data;
            match feed.update(&mut txn).await {
                Ok(()) => {
                    txn.commit().await?;
                    Ok([("HX-Redirect", "/")].into_response())
                }
                Err(error) => Ok(feed_form(
                    Some(&form_values),
                    ValidationErrors::from_feed_update_error(error),
                )
                .into_response()),
            }
        }
        Err(errors) => Ok(feed_form(Some(&form_values), errors).into_response()),
    }
}

#[axum::debug_handler]
pub async fn feed_create_get() -> ServerResult<Response> {
    Ok(layout(html!(
        h2 { "Create Feed" }
        (feed_form(None, ValidationErrors::default()))
    ))
    .into_response())
}

#[axum::debug_handler]
pub async fn feed_create_post(
    State(pool): State<SqlitePool>,
    TypedMultipart(form_values): TypedMultipart<FeedFormValues>,
) -> ServerResult<Response> {
    match form_values.try_into_feed_data() {
        Ok(feed_data) => {
            let mut txn = pool.begin().await?;
            feed_data.create(&mut txn).await?;
            txn.commit().await?;

            Ok([("HX-Redirect", "/")].into_response())
        }
        Err(errors) => Ok(feed_form(Some(&form_values), errors).into_response()),
    }
}

#[axum::debug_handler]
pub async fn test(Authenticated: Authenticated) -> ServerResult<Response> {
    Ok(layout(html!(
        h2 { "Hello World!" }
    ))
    .into_response())
}
