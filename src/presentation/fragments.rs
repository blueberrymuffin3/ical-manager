use std::{fmt::{self, Display}, convert::Infallible};

use anyhow::Context;
use icondata::LuIcon;
use maud::{html, Markup};
use sqlx::SqlitePool;

use crate::data::feed::Feed;

use super::icon::icon;

fn feed_row(id: i64, name: &str, link_code: &str) -> Markup {
    html!(tr {
        td { (name) }
        td { (feed_status_loader(id)) }
        td.actions {
            a."button"."small-button"."round-button"
                href=(format_args!("/feed/{id}/edit")) {
                (icon(LuIcon::LuEdit))
                " Edit"
            }

            button."small-button"."muted-button"."round-button"
                onclick="copyFeedLink(this);"
            {
                (icon(LuIcon::LuCopy))
                " Copy Public Link"
            }

            button."small-button"."danger-button"."round-button" {
                (icon(LuIcon::LuTrash))
                " Delete"
            }

            input type="text" class="copy-source" data-partial-copy-uri=(format_args!("/export/{link_code}.ical")) readonly {}
        }
    })
}

pub fn feed_table(feeds: &[Feed]) -> Markup {
    if feeds.is_empty() {
        return html!(
            p."text-center" {
                "No feeds found"
            }
        );
    }

    html!(
        table."striped-table" id="feeds-table" {
            thead {
                tr {
                    th { "Name" }
                    th { "Status" }
                    th { "Actions" }
                }
            }
            tbody {
                @for feed in feeds {
                    (feed_row(feed.id, &feed.data.name, &feed.link_code))
                }
            }
        }
    )
}

enum FeedStatus {
    Loading,
    Error,
    Warn,
    Ok,
}

fn feed_status_id(id: impl Display) -> String {
    format!("feed-status-{id}")
}

fn feed_status_base(
    status: FeedStatus,
    id: i64,
    text: &str,
    hx_get: Option<fmt::Arguments>,
) -> Markup {
    let type_class = match status {
        FeedStatus::Loading => "status-loading",
        FeedStatus::Error => "status-error",
        FeedStatus::Warn => "status-warn",
        FeedStatus::Ok => "status-ok",
    };
    html!(
        div
            .status.(type_class)
            #(feed_status_id(id))
            hx-get=[hx_get]
            hx-swap=[hx_get.and(Some("outerHTML"))]
            hx-trigger=[hx_get.and(Some("load"))]
        { (text) }
    )
}

fn feed_status_loader(id: i64) -> Markup {
    feed_status_base(
        FeedStatus::Loading,
        id,
        "Loading...",
        Some(format_args!("/feed/{id}/status")),
    )
}

fn feed_status_result(status: FeedStatus, id: i64, text: &str) -> Markup {
    feed_status_base(status, id, text, None)
}

pub async fn feed_status(pool: &SqlitePool, id: i64) -> anyhow::Result<Markup> {
    let Some(feed) = sqlx::query!("SELECT id FROM Feed WHERE id = ?", id)
        .fetch_optional(pool)
        .await
        .context("Error fetching feed list")?
    else {
        return Ok(feed_status_result(FeedStatus::Error, id, "Not Found"));
    };

    return Ok(feed_status_result(FeedStatus::Ok, feed.id, "Found"));
}
