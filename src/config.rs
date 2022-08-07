use std::{path::Path, sync::Arc};

use anyhow::Context;
use reqwest::ClientBuilder;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub tmdb: TmdbConfig,
    pub show: Vec<UserShow>,
    pub server: ServerConfig,
}

impl Config {
    pub async fn load(path: impl AsRef<Path>) -> Result<Self, anyhow::Error> {
        let path = path.as_ref();
        let config_s = tokio::fs::read_to_string(path)
            .await
            .with_context(|| format!("Cannot load config from location \"{}\"", path.display()))?;
        let c = toml::from_str(&config_s)?;
        Ok(c)
    }
}

#[derive(Debug, Deserialize)]
pub struct TmdbConfig {
    v4_auth: String,
    language: Option<String>,
    proxy: Option<String>,
    timeout_s: Option<u64>,
}

impl TmdbConfig {
    pub fn client(&self) -> Result<crate::tmdb::Client, reqwest::Error> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", self.v4_auth).parse().unwrap(),
        );
        let mut builder = ClientBuilder::new().default_headers(headers);
        if let Some(proxy) = &self.proxy {
            tracing::info!(%proxy, "using proxy");
            builder = builder.proxy(reqwest::Proxy::all(proxy)?);
        }
        if let Some(timeout_s) = self.timeout_s {
            builder = builder.timeout(std::time::Duration::from_secs(timeout_s));
        }

        let client = builder.build()?;

        Ok(crate::tmdb::Client {
            inner: client,
            language: self.language.clone().map(|s| s.into()),
        })
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct UserShow {
    pub id: u64,
    pub offset_days: Option<i32>,
    pub url: Option<Arc<str>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    #[serde(default)]
    pub tcp: Vec<std::net::SocketAddr>,
    #[serde(default)]
    pub uds: Vec<std::path::PathBuf>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn parse_config() {
        let _c = Config::load("./examples/config.toml").await.unwrap();
    }
}
