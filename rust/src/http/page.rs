use super::repo_url::Url;
use crate::prelude::*;
use scraper::{Html, Selector};

const USER_AGENT: &str = "digraph/0.1.0";

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
        if !self.should_fetch() {
            log::info!(
                "document not suitable for fetching, skipping fetch: {}",
                self.0
            );
            return Ok(Response {
                url: self.0.clone(),
                body: Html::parse_document("<title>Missing title [pdf]</title>"),
            });
        }

        log::info!("Fetching page at link {}", self.0);
        let client = reqwest::Client::builder()
            .user_agent(USER_AGENT)
            // We're just interested in the link title for now, so this is hopefully not an unsafe
            // operation in our context.  The user's browser can take over when the user attempts
            // to follow the link.
            .danger_accept_invalid_certs(true)
            .build()?;

        let text = client
            .get(self.0.normalized.clone())
            .send()
            .await?
            .text()
            .await?;
        let body = Html::parse_fragment(text.as_ref());

        Ok(Response {
            url: self.0.clone(),
            body,
        })
    }

    pub fn should_fetch(&self) -> bool {
        !self.0.is_pdf()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_should_fetch() {
        let url =
            Url::parse("https://www.dni.gov//Prelimary-Assessment-UAP-20210625.pdf?q=something")
                .unwrap();
        let page = Page::from(url);
        assert!(!page.should_fetch());

        let url =
            Url::parse("https://www.dni.gov//Prelimary-Assessment-UAP-20210625.html?q=something")
                .unwrap();
        let page = Page::from(url);
        assert!(page.should_fetch());
    }
}
