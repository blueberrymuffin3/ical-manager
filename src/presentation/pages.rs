use anyhow::Context;
use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
};
use axum_typed_multipart::TypedMultipart;
use http::{header::CONTENT_TYPE, StatusCode};
use icondata::LuIcon;
use maud::{html, Markup};
use sqlx::SqlitePool;

use crate::{data::feed::Feed, logic::process_feed, AppState};

use super::{
    auth::Authenticated,
    error::{make_404, make_error_page_auto, ServerResult},
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
        (feed_form(Some(&values), false, ValidationErrors::default()))
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
                    false,
                    ValidationErrors::from_feed_update_error(error),
                )
                .into_response()),
            }
        }
        Err(errors) => Ok(feed_form(Some(&form_values), false, errors).into_response()),
    }
}

#[axum::debug_handler]
pub async fn feed_create_get() -> ServerResult<Response> {
    Ok(layout(html!(
        h2 { "Create Feed" }
        (feed_form(None, true, ValidationErrors::default()))
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
        Err(errors) => Ok(feed_form(Some(&form_values), true, errors).into_response()),
    }
}

#[axum::debug_handler(state = AppState)]
pub async fn test(Authenticated(user): Authenticated) -> ServerResult<Response> {
    Ok(layout(html!(
        p {
            "Hello, "
            img src=[user.data.icon] width="16" {}
            " "
            (user.data.name.as_deref().unwrap_or("Unknown"))
            "!"
        }
    ))
    .into_response())
}

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
            format_args!("No feed found with link code {code:?}"),
            StatusCode::NOT_FOUND,
        ));
    };

    let (data, _stats) = process_feed(&feed, &pool).await?;

    Ok(([(CONTENT_TYPE, "text/calendar")], data).into_response())
}
