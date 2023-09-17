use anyhow::{Context, bail};
use async_trait::async_trait;
use bytes::Bytes;

use crate::data::{source::{Source, SourceFile, SourceHTTP}, cache::FetchCacheEntry};

#[async_trait]
pub trait SourceTrait {
    async fn fetch(&self) -> anyhow::Result<Bytes>;
}

#[async_trait]
impl SourceTrait for Source {
    async fn fetch(&self) -> anyhow::Result<Bytes> {
        match self {
            Source::HTTP(http) => http.fetch().await,
            Source::File(file) => file.fetch().await,
        }
    }
}

#[async_trait]
impl SourceTrait for SourceHTTP {
    async fn fetch(&self) -> anyhow::Result<Bytes> {
        // TODO: Make this more robust, check mime types, set headers, etc...
        let response = reqwest::get(&self.link)
            .await
            .with_context(|| format!("Error fetching {}", self.link))?;
        Ok(response
            .bytes()
            .await
            .context("Error downloading response")?)
    }
}

#[async_trait]
impl SourceTrait for SourceFile {
    async fn fetch(&self) -> anyhow::Result<Bytes> {
        bail!("Uploaded file is missing");
    }
}
