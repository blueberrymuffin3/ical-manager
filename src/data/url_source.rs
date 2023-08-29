use std::{fmt::Display, str::FromStr};

#[derive(Clone, Copy, Debug, sqlx::Type)]
#[sqlx(transparent)]
pub struct FeedTtl(i64);

impl Display for FeedTtl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        humantime::format_duration(std::time::Duration::from_secs(self.0.try_into().unwrap()))
            .fmt(f)
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
    pub ttl_seconds: FeedTtl,
}
