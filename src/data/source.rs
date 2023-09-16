mod ttl;

use anyhow::{anyhow, bail};
use bytes::Bytes;
use sqlx::{decode::Decode, types::Type, ColumnIndex, FromRow, Row, Sqlite, Transaction};

use super::feed::FeedUpdateError;

#[derive(Debug, Clone)]
pub struct SourceHTTP {
    pub link: String,
}

#[derive(Debug, Clone)]
pub struct SourceFile {
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
            sqlx::query_as!(SourceHTTP, "SELECT link FROM SourceHTTP WHERE id = ?", id)
                .fetch_optional(&mut *txn)
                .await?,
            sqlx::query!("SELECT contents FROM SourceFile WHERE id = ?", id)
                .fetch_optional(&mut *txn)
                .await?,
        ) {
            (Some(source_http), None) => Ok(Self::HTTP(source_http)),
            (None, Some(source_file)) => Ok(Self::File(SourceFile {
                contents: Some(source_file.contents.into()),
            })),
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
            Source::HTTP(SourceHTTP { link }) => {
                sqlx::query!("INSERT INTO SourceHttp(id, link) VALUES (?, ?) ON CONFLICT(id) DO UPDATE SET link = excluded.link", id, link)
                    .fetch_all(&mut *txn)
                    .await?;
            }
            Source::File(SourceFile { contents }) => {
                match contents {
                    Some(contents) => {
                        let contents = contents.to_vec();
                        sqlx::query!(
                            "INSERT INTO SourceFile(id, contents) VALUES (?, ?) ON CONFLICT(id) DO UPDATE SET contents = excluded.contents",
                            id,
                            contents
                        )
                        .fetch_all(&mut *txn)
                        .await?;
                    }
                    None => {
                        sqlx::query_scalar!("SELECT id FROM SourceFile WHERE id = ?", id)
                            .fetch_optional(&mut *txn)
                            .await?
                            .ok_or(FeedUpdateError::FileSourceMissingFileError)?;
                    }
                }

                // if let Some(contents) = contents {
                //     let contents = contents.to_vec();
                //     sqlx::query!(
                //         "UPDATE SourceFile SET contents = ? WHERE id = ?",
                //         contents,
                //         id,
                //     )
                //     .fetch_all(&mut *txn)
                //     .await?;
                // }
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

        // match (source_link, ttl_seconds) {
        //     (Some(source_link), Some(ttl)) => Ok(Source::Url(SourceUrl { source_link, ttl })),
        //     (None, None) => Ok(Source::File),
        //     _ => Err(sqlx::Error::ColumnDecode {
        //         index: "source_link".to_owned(),
        //         source: anyhow!("Could not decode source from database").into(),
        //     }),
        // }
    }
}
