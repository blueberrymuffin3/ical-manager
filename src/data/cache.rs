use bytes::Bytes;
use chrono::{DateTime, Utc};
use sqlx::{Executor, FromRow, Sqlite};

#[derive(Debug, Clone, FromRow)]
pub struct FetchCacheEntry {
    pub id: i64,
    pub timestamp: DateTime<Utc>,
    pub data: Bytes,
}

impl FetchCacheEntry {
    pub async fn select_by_id(
        id: i64,
        executor: impl Executor<'_, Database = Sqlite>,
    ) -> sqlx::Result<Option<FetchCacheEntry>> {
        let Some(record) = sqlx::query!(
            "SELECT id, timestamp as 'timestamp: DateTime<Utc>', data FROM FetchCache WHERE id = ?",
            id,
        )
        .fetch_optional(executor)
        .await?
        else {
            return Ok(None);
        };

        Ok(Some(FetchCacheEntry {
            id: record.id,
            timestamp: record.timestamp,
            data: Bytes::from(record.data),
        }))
    }

    pub async fn upsert(&self, executor: impl Executor<'_, Database = Sqlite>) -> sqlx::Result<()> {
        let data = self.data.as_ref();
        sqlx::query!(
            "INSERT INTO FetchCache(id, timestamp, data) VALUES(?, ?, ?) ON CONFLICT(id) DO UPDATE SET timestamp = excluded.timestamp, data = excluded.data",
            self.id, self.timestamp,
            data,
        ).fetch_all(executor).await?;
        Ok(())
    }
}
