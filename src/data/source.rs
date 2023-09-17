use anyhow::{anyhow, bail};
use bytes::Bytes;
use chrono::Utc;
use sqlx::{decode::Decode, types::Type, ColumnIndex, FromRow, Row, Sqlite, Transaction};

use super::{cache::FetchCacheEntry, feed::FeedUpdateError, ttl::SourceTTL};

#[derive(Debug, Clone)]
pub struct SourceHTTP {
    pub link: String,
    pub ttl: SourceTTL,
}

#[derive(Debug, Clone)]
pub struct SourceFile {
    /// Only used for upserts, always None for selects
    pub contents: Option<Bytes>,
}

#[derive(Debug, Clone)]
pub enum Source {
    HTTP(SourceHTTP),
    File(SourceFile),
}

impl Source {
    pub(super) async fn select(txn: &mut Transaction<'_, Sqlite>, id: i64) -> anyhow::Result<Self> {
        match (
            sqlx::query_as!(
                SourceHTTP,
                "SELECT link, ttl as 'ttl: SourceTTL' FROM SourceHTTP WHERE id = ?",
                id
            )
            .fetch_optional(&mut *txn)
            .await?,
            sqlx::query!("SELECT id FROM SourceFile WHERE id = ?", id)
                .fetch_optional(&mut *txn)
                .await?,
        ) {
            (Some(source_http), None) => Ok(Self::HTTP(source_http)),
            (None, Some(_)) => Ok(Self::File(SourceFile { contents: None })),
            _ => bail!("Could not decode source from database: source count != 1"),
        }
    }

    pub(super) async fn upsert(
        &self,
        txn: &mut Transaction<'_, Sqlite>,
        id: i64,
    ) -> Result<(), FeedUpdateError> {
        if !matches!(self, Self::HTTP(_)) {
            sqlx::query!("DELETE FROM SourceHttp WHERE id = ?", id)
                .fetch_all(&mut *txn)
                .await?;
        }

        if !matches!(self, Self::File(_)) {
            sqlx::query!("DELETE FROM SourceFile WHERE id = ?", id)
                .fetch_all(&mut *txn)
                .await?;
        }

        match self {
            Source::HTTP(SourceHTTP { link, ttl }) => {
                sqlx::query!(
                    "INSERT INTO SourceHttp(id, link, ttl) VALUES (?, ?, ?) ON CONFLICT(id) DO UPDATE SET link = excluded.link, ttl = excluded.ttl",
                    id, link, ttl
                )
                    .fetch_all(&mut *txn)
                    .await?;
            }
            Source::File(SourceFile { contents }) => {
                sqlx::query!(
                    "INSERT INTO SourceFile(id) VALUES (?) ON CONFLICT(id) DO NOTHING",
                    id,
                )
                .fetch_all(&mut *txn)
                .await?;

                if let Some(contents) = contents {
                    FetchCacheEntry {
                        id,
                        timestamp: Utc::now(),
                        data: contents.clone(),
                    }
                    .upsert(&mut *txn)
                    .await?;
                }
            }
        }

        Ok(())
    }
}

impl<'a, R: Row> FromRow<'a, R> for Source
where
    &'a str: ColumnIndex<R>,
    Option<i64>: Decode<'a, R::Database> + Type<R::Database>,
    SourceHTTP: FromRow<'a, R>,
    SourceFile: FromRow<'a, R>,
{
    fn from_row(row: &'a R) -> ::sqlx::Result<Self> {
        match (
            row.try_get("source_url_id")?,
            row.try_get("source_file_id")?,
        ) {
            (Some(_), None) => SourceHTTP::from_row(row).map(Source::HTTP),
            (None, Some(_)) => SourceFile::from_row(row).map(Source::File),
            _ => Err(sqlx::Error::Decode(
                anyhow!("Could not decode source from database: source count != 1").into(),
            )),
        }
    }
}
