use std::sync::Arc;

use reqwest::Url;

use crate::models;

#[derive(Clone)]
pub struct Client {
    pub(crate) inner: reqwest::Client,
    pub(crate) language: Option<Arc<str>>,
}

impl Client {
    fn url(&self, path: &str) -> Url {
        debug_assert!(path.starts_with('/'));
        let mut url = Url::parse("https://api.themoviedb.org/").unwrap();
        url.set_path(format!("3{}", path).as_str());
        if let Some(l) = &self.language {
            url.query_pairs_mut().append_pair("language", l);
        };
        url
    }
    pub async fn get_show(&self, id: u64) -> Result<models::Show, reqwest::Error> {
        let url = self.url(&format!("/tv/{id}"));
        let response = self.inner.get(url).send().await?;
        let show: models::Show = response.json().await?;
        Ok(show)
    }
    pub async fn get_season(
        &self,
        show_id: u64,
        season_number: u32,
    ) -> Result<models::Season, reqwest::Error> {
        let url = self.url(&format!(
            "/tv/{id}/season/{season_number}",
            id = show_id,
            season_number = season_number
        ));
        let response = self.inner.get(url).send().await?;
        let season: models::Season = response.json().await?;
        Ok(season)
    }
}
