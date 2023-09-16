use std::fmt;

use icondata::LuIcon;
use maud::{html, Markup, Render};
use sqlx::SqlitePool;

use crate::{
    data::feed::Feed,
    logic::{process_feed, CalendarStats},
};

use super::icon::icon;

fn feed_row(id: i64, name: &str, link_code: &str) -> Markup {
    html!(tr {
        td { (name) }
        td hx-target="this" {
            (feed_status_loader(id))
        }
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
                " Link"
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
            p."text-center" #"feeds-table" {
                "No feeds found"
            }
        );
    }

    html!(
        table."striped-table" #"feeds-table" {
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

fn feed_status_base(
    status: FeedStatus,
    text: impl Render,
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
            hx-get=[hx_get]
            hx-trigger=[hx_get.and(Some("load"))]
        { (text) }
    )
}

fn feed_status_loader(id: i64) -> Markup {
    feed_status_base(
        FeedStatus::Loading,
        "Loading...",
        Some(format_args!("/feed/{id}/status")),
    )
}

fn feed_status_result(status: FeedStatus, text: impl Render) -> Markup {
    feed_status_base(status, text, None)
}

pub async fn feed_status(pool: &SqlitePool, id: i64) -> anyhow::Result<Markup> {
    match get_feed_status(pool, id).await {
        Ok(Some((_data, stats))) => Ok(feed_status_result(
            FeedStatus::Ok,
            format_args!("Updated now, {} events", stats.event_count),
        )),
        Ok(None) => Ok(feed_status_result(FeedStatus::Error, "Not Found")),
        Err(err) => Ok(feed_status_result(FeedStatus::Error, format_args!("{err}"))),
    }
}

async fn get_feed_status(
    pool: &sqlx::Pool<sqlx::Sqlite>,
    id: i64,
) -> Result<Option<(bytes::Bytes, CalendarStats)>, anyhow::Error> {
    let mut txn = pool.begin().await?;
    let feed = Feed::select_by_id(id, &mut txn).await?;
    txn.rollback().await?;

    match feed {
        Some(feed) => Ok(Some(process_feed(&feed).await?)),
        None => Ok(None),
    }
}
