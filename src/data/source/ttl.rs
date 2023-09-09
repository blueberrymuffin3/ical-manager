use std::str::FromStr;

use std;

use std::fmt::Display;

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
