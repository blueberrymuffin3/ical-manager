use anyhow::Context;
use sqlx::{Executor, FromRow, Sqlite, Transaction};
use uuid::Uuid;

use super::{
    filters::{self, Filters},
    source::{self, Source},
};

#[derive(Debug, Clone, FromRow)]
pub struct Feed {
    pub id: i64,
    pub link_code: String,
    #[sqlx(flatten)]
    pub data: FeedData,
}

#[derive(Debug, Clone)]
pub struct FeedData {
    pub name: String,
    pub source: source::Source,
    pub filters: filters::Filters,
}

struct FeedRecord {
    id: i64,
    link_code: String,
    name: String,
}

#[derive(thiserror::Error, Debug)]
pub enum FeedUpdateError {
    #[error("Internal Error")]
    InternalError(#[from] anyhow::Error),
    #[error("Database Error")]
    DatabaseError(#[from] sqlx::Error),
    #[error("No existing file uploaded, please upload one")]
    FileSourceMissingFileError,
}

impl FeedData {
    pub async fn create(self, txn: &mut Transaction<'_, Sqlite>, user_id: i64) -> Result<Feed, FeedUpdateError> {
        let link_code = Uuid::new_v4().simple().to_string();
        let id = sqlx::query_scalar!(
            "INSERT INTO Feed(user_id, link_code, name) VALUES (?, ?, ?) RETURNING id",
            user_id,
            link_code,
            self.name,
        )
        .fetch_one(&mut *txn)
        .await?;

        self.source.upsert(&mut *txn, id).await?;
        self.filters.upsert(&mut *txn, id).await?;

        Ok(Feed {
            id,
            link_code,
            data: self,
        })
    }
}

impl Feed {
    pub async fn select(
        txn: &mut Transaction<'_, Sqlite>,
        user_id: i64,
    ) -> anyhow::Result<Vec<Feed>> {
        let records = sqlx::query_as!(
            FeedRecord,
            "SELECT id, link_code, name FROM Feed WHERE user_id = ? ORDER BY id",
            user_id
        )
        .fetch_all(&mut *txn)
        .await?;
        let mut vec = Vec::with_capacity(records.len());

        for record in records {
            vec.push(Feed::from_record(&mut *txn, record).await?)
        }

        Ok(vec)
    }

    pub async fn select_by_id(
        user_id: i64,
        id: i64,
        txn: &mut Transaction<'_, Sqlite>,
    ) -> anyhow::Result<Option<Feed>> {
        let Some(record) = sqlx::query_as!(
            FeedRecord,
            "SELECT id, link_code, name FROM Feed WHERE user_id = ? AND id = ?",
            user_id,
            id
        )
        .fetch_optional(&mut *txn)
        .await?
        else {
            return Ok(None);
        };

        Ok(Some(Feed::from_record(&mut *txn, record).await?))
    }

    pub async fn select_by_link_code(
        link_code: &str,
        txn: &mut Transaction<'_, Sqlite>,
    ) -> anyhow::Result<Option<Feed>> {
        let Some(record) = sqlx::query_as!(
            FeedRecord,
            "SELECT id, link_code, name FROM Feed WHERE link_code = ?",
            link_code
        )
        .fetch_optional(&mut *txn)
        .await?
        else {
            return Ok(None);
        };

        Ok(Some(Feed::from_record(&mut *txn, record).await?))
    }

    pub async fn update(&self, txn: &mut Transaction<'_, Sqlite>) -> Result<(), FeedUpdateError> {
        let id = sqlx::query_scalar!(
            "UPDATE Feed SET link_code = ?, name = ? WHERE id = ? RETURNING id",
            self.link_code,
            self.data.name,
            self.id,
        )
        .fetch_one(&mut *txn)
        .await
        .context("Error updating feed")?
        .expect("NOT NULL violated");

        self.data.source.upsert(&mut *txn, id).await?;
        self.data.filters.upsert(&mut *txn, id).await?;

        Ok(())
    }

    pub async fn delete_by_id(
        user_id: i64,
        id: i64,
        executor: impl Executor<'_, Database = Sqlite>,
    ) -> sqlx::Result<bool> {
        Ok(sqlx::query_scalar!("DELETE FROM Feed WHERE user_id = ? AND id = ? RETURNING id", user_id, id)
            .fetch_optional(executor)
            .await?
            .is_some())
    }

    async fn from_record(
        txn: &mut Transaction<'_, Sqlite>,
        feed_record: FeedRecord,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            id: feed_record.id,
            link_code: feed_record.link_code,
            data: FeedData {
                name: feed_record.name,
                source: Source::select(txn, feed_record.id)
                    .await
                    .context("Error fetching source for feed")?,
                filters: Filters::select(txn, feed_record.id)
                    .await
                    .context("Error fetching filters for feed")?,
            },
        })
    }
}
