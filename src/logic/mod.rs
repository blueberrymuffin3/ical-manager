mod fetch;
mod filters;
mod ssrf_guard;

use anyhow::Context;
use bytes::Bytes;
use chrono::{Duration, Utc};
use sqlx::{Executor, Sqlite};

use crate::data::{cache::FetchCacheEntry, feed::Feed};

use fetch::SourceTrait;

use self::filters::{apply_filters, FilterStats};

#[derive(Debug)]
pub struct CalendarStats {
    pub event_count: usize,
    pub size: usize,
    pub cache_age: Duration,
}

pub async fn process_feed(
    feed: &Feed,
    executor: impl Executor<'_, Database = Sqlite> + Copy,
) -> anyhow::Result<(Bytes, CalendarStats)> {
    let cache_entry = FetchCacheEntry::select_by_id(feed.id, executor).await?;

    let (data, cache_age) = match cache_entry {
        Some(FetchCacheEntry {
            id: _,
            timestamp,
            data: cache_data,
        }) => {
            let age = Utc::now() - timestamp;

            if feed.data.source.is_expired(age) {
                (None, Duration::zero())
            } else {
                (Some(cache_data), age)
            }
        }
        None => (None, Duration::zero()),
    };

    let data = match data {
        None => {
            let data = feed
                .data
                .source
                .fetch()
                .await
                .context("Error fetching feed")?;

            FetchCacheEntry {
                id: feed.id,
                timestamp: Utc::now(),
                data: data.clone(),
            }
            .upsert(executor)
            .await?;

            data
        }
        Some(data) => data,
    };

    let (data, FilterStats { event_count, size }) =
        apply_filters(data, &feed.data.filters).context("Error processing feed contents")?;

    Ok((
        data,
        CalendarStats {
            cache_age,
            event_count,
            size,
        },
    ))
}
