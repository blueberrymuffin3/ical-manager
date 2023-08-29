use std::{
    fmt::{self, Display},
    time::Duration,
};

use anyhow::Context;
use icondata::LuIcon;
use maud::{html, Markup};
use sqlx::SqlitePool;
use strum::{EnumString, IntoStaticStr};

use super::icon::icon;

fn feed_row(id: i64, name: &str, link_code: &str) -> anyhow::Result<Markup> {
    Ok(html!(tr {
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

            input type="text" class="copy-source" data-partial-copy-uri=(format_args!("/export/{link_code}.ical")) readonly {}
        }
    }))
}

pub async fn feed_table(pool: &SqlitePool) -> anyhow::Result<Markup> {
    let feeds = sqlx::query!("SELECT id, name, link_code FROM Feed")
        .fetch_all(pool)
        .await
        .context("Error fetching feed list")?;

    if feeds.is_empty() {
        return Ok(html!(
            p."text-center" {
                "No feeds found"
            }
        ));
    }

    Ok(html!(
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
                    (feed_row(feed.id, &feed.name, &feed.link_code)?)
                }
            }
        }
    ))
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

#[derive(Clone, Copy, Debug, PartialEq, EnumString, IntoStaticStr)]
#[strum(serialize_all = "kebab-case")]
pub enum FeedType {
    Upload,
    Link,
}

#[derive(Debug)]
pub struct FormValues {
    pub name: String,
    pub feed_type: FeedType,
    pub link: String,
    pub ttl: Option<Duration>,
}

fn select_options(selected_value: Option<&str>, options: &[(&str, &str)]) -> Markup {
    html!(
        @for (value, label) in options {
            option value=(value) selected[selected_value == Some(value)] {(label)}
        }
    )
}

const DURATION_6_HOURS: Duration = Duration::from_secs(6 * 60 * 60);

pub fn feed_form(values: Option<&FormValues>) -> Markup {
    let (name, feed_type, link, ttl) = match values {
        Some(FormValues {
            name,
            feed_type,
            link,
            ttl,
        }) => (
            Some(name.as_str()),
            Some(feed_type),
            Some(link.as_str()),
            *ttl,
        ),
        None => (None, None, None, None),
    };

    let ttl = humantime::format_duration(ttl.unwrap_or(DURATION_6_HOURS));

    html!(
        form #"form-form" {
            h3 {"Source"}

            label for="name" {"Name"}
            input #name type="text" value=[name];

            label for="type" {"Type"}
            select #type onchange="updateFormForm();" value=[feed_type.map(<&str>::from)] {
                (select_options(
                    feed_type.map(<&str>::from),
                &[
                    ("link", "Fetch URL"),
                    ("upload", "File Upload"),
                ]))
            }

            #"section-link".hide {
                label for="link" {"Link"}
                input #link type="text" placeholder="https://example.com/feed.ical" value=[link];

                label for="ttl" {"Minimum Update Period"}
                input #ttl type="text" value=(ttl) ;
                p {"Enter a value like 1 day, 15min, 3h, etc..."}
            }

            #"section-upload".hide {
                label for="upload" {"Upload iCal"}
                input #upload type="file";
            }

            h3 {"Filters"}
            label for="filter-cr" {
                input #"filter-cr" type="checkbox";
                " Remove Carriage Returns (" code {"\\r"} ")"
            }

            input type="submit";
        }

    )
}
