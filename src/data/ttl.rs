use std::fmt::{self, Display};
use std::str::FromStr;

use chrono::Duration;

#[derive(Clone, Copy, Debug, sqlx::Type)]
#[sqlx(transparent)]
pub struct SourceTTL(i64);

impl SourceTTL {
    pub fn duration(&self) -> Duration {
        Duration::seconds(self.0)
    }
}

impl Display for SourceTTL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        humantime::format_duration(std::time::Duration::from_secs(self.0.try_into().unwrap()))
            .fmt(f)
    }
}

impl Default for SourceTTL {
    fn default() -> Self {
        // 1 hour
        Self(60 * 60)
    }
}

impl FromStr for SourceTTL {
    type Err = humantime::DurationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            humantime::parse_duration(s)?.as_secs().try_into().unwrap(),
        ))
    }
}
