use reqwest::Url;

use crate::models;

pub struct Client {
    inner: reqwest::Client,
    language: Option<String>,
}

impl Client {
    pub fn new(token_v4: &str, language: Option<String>) -> Result<Self, reqwest::Error> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", token_v4).parse().unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;
        Ok(Self {
            inner: client,
            language,
        })
    }
    fn url(&self, path: &str) -> Url {
        debug_assert!(path.starts_with('/'));
        let mut url = Url::parse("https://api.themoviedb.org/").unwrap();
        url.set_path(format!("3{}", path).as_str());
        if let Some(l) = &self.language {
            url.query_pairs_mut().append_pair("language", l.as_str());
        };
        url
    }
    pub async fn get_show(&self, id: u64) -> Result<models::Show, reqwest::Error> {
        let url = self.url(&format!("/tv/{}", id));
        let response = self.inner.get(url).send().await?;
        let show: models::Show = response.json().await?;
        Ok(show)
    }
}
