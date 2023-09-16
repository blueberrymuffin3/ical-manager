mod fetch;
mod filters;

use anyhow::Context;
use bytes::Bytes;

use crate::data::feed::Feed;

use fetch::SourceTrait;

use self::filters::apply_filters;

#[derive(Debug)]
pub struct CalendarStats {
    pub event_count: usize,
    pub size: usize,
}

pub async fn process_feed(feed: &Feed) -> anyhow::Result<(Bytes, CalendarStats)> {
    let data = feed
        .data
        .source
        .fetch()
        .await
        .context("Error fetching feed")?;

    let result =
        apply_filters(data, &feed.data.filters).context("Error processing feed contents")?;

    Ok(result)
}
