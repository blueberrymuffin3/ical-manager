use std::sync::Arc;

use anyhow::bail;
use async_trait::async_trait;
use bytes::Bytes;
use once_cell::sync::Lazy;
use reqwest::{Client, ClientBuilder};
use url::Url;

use crate::data::source::{Source, SourceFile, SourceHTTP};

use super::ssrf_guard::{check_url, ssrf_safe_redirect_policy, SSRFSafeResolver};

#[async_trait]
pub trait SourceTrait {
    fn is_expired(&self, age: chrono::Duration) -> bool;
    async fn fetch(&self) -> anyhow::Result<Bytes>;
}

#[async_trait]
impl SourceTrait for Source {
    fn is_expired(&self, age: chrono::Duration) -> bool {
        match self {
            Source::HTTP(http) => http.is_expired(age),
            Source::File(file) => file.is_expired(age),
        }
    }

    async fn fetch(&self) -> anyhow::Result<Bytes> {
        match self {
            Source::HTTP(http) => http.fetch().await,
            Source::File(file) => file.fetch().await,
        }
    }
}

const HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    ClientBuilder::new()
        .timeout(std::time::Duration::from_secs(20))
        .dns_resolver(Arc::new(SSRFSafeResolver))
        .redirect(ssrf_safe_redirect_policy())
        .user_agent(concat!(
            env!("CARGO_PKG_NAME"),
            "/",
            env!("CARGO_PKG_VERSION"),
        ))
        .build()
        .expect("Invalid reqwest client config")
});

#[async_trait]
impl SourceTrait for SourceHTTP {
    fn is_expired(&self, age: chrono::Duration) -> bool {
        age > self.ttl.duration()
    }

    async fn fetch(&self) -> anyhow::Result<Bytes> {
        let url: Url = self.link.parse()?;
        check_url(&url)?;

        let response = HTTP_CLIENT.get(url).send().await?;
        if !response.status().is_success() {
            bail!("Unexpected response code: {}", response.status())
        }

        Ok(response.bytes().await?)
    }
}

#[async_trait]
impl SourceTrait for SourceFile {
    fn is_expired(&self, _age: chrono::Duration) -> bool {
        false
    }

    async fn fetch(&self) -> anyhow::Result<Bytes> {
        bail!("Uploaded file is missing");
    }
}
