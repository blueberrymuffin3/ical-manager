use std::{fmt::Display, str::FromStr};

use anyhow::anyhow;

#[derive(Clone, Copy, Debug, sqlx::Type)]
#[sqlx(transparent)]
pub struct FeedTtl(i64);

impl FeedTtl {
    pub const fn from_seconds(seconds: i64) -> Self {
        assert!(seconds > 0);
        Self(seconds)
    }
}

impl Display for FeedTtl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        humantime::format_duration(std::time::Duration::from_secs(self.0.try_into().unwrap()))
            .fmt(f)
    }
}

impl Default for FeedTtl {
    fn default() -> Self {
        // 1 hour
        Self(60 * 60)
    }
}

impl FromStr for FeedTtl {
    type Err = humantime::DurationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            humantime::parse_duration(s)?.as_secs().try_into().unwrap(),
        ))
    }
}

#[derive(Debug, Clone)]
pub struct UrlSource {
    pub source_link: String,
    pub ttl: FeedTtl,
}

#[derive(Debug, Clone)]
pub enum Source {
    UrlSource(UrlSource),
    FileSource,
}

impl<'a, R: ::sqlx::Row> ::sqlx::FromRow<'a, R> for Source
where
    &'a ::std::primitive::str: ::sqlx::ColumnIndex<R>,
    Option<String>: ::sqlx::decode::Decode<'a, R::Database>,
    Option<String>: ::sqlx::types::Type<R::Database>,
    Option<FeedTtl>: ::sqlx::decode::Decode<'a, R::Database>,
    Option<FeedTtl>: ::sqlx::types::Type<R::Database>,
{
    fn from_row(row: &'a R) -> ::sqlx::Result<Self> {
        let source_link: Option<String> = row.try_get("source_link")?;
        let ttl_seconds: Option<FeedTtl> = row.try_get("ttl_seconds")?;
        match (source_link, ttl_seconds) {
            (Some(source_link), Some(ttl)) => Ok(Source::UrlSource(UrlSource { source_link, ttl })),
            (None, None) => Ok(Source::FileSource),
            _ => Err(sqlx::Error::ColumnDecode {
                index: "source_link".to_owned(),
                source: anyhow!("Could not decode source from database").into(),
            }),
        }
    }
}
