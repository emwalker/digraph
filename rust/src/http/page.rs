use super::repo_url::Url;
use crate::prelude::*;
use scraper::{Html, Selector};

#[derive(Debug)]
pub struct Page(pub Url);

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Response {
    url: Url,
    #[derivative(Debug = "ignore")]
    body: Html,
}

impl Response {
    pub fn title(&self) -> Option<String> {
        let sel = Selector::parse("title").expect("failed to parse selector");
        self.body
            .select(&sel)
            .next()
            .map(|element| element.inner_html())
    }
}

impl Page {
    pub fn from(url: Url) -> Self {
        Self(url)
    }

    pub async fn fetch(&self) -> Result<Response> {
        log::info!("Fetching page at link {}", self.0);
        let text = reqwest::get(self.0.normalized.clone())
            .await?
            .text()
            .await?;
        let body = Html::parse_fragment(text.as_ref());

        Ok(Response {
            url: self.0.clone(),
            body,
        })
    }
}
