use async_trait::async_trait;

use crate::prelude::*;

mod page;
pub use page::*;
mod repo_url;
pub use repo_url::*;

#[async_trait]
pub trait Fetch {
    async fn fetch(&self, url: &repo_url::RepoUrl) -> Result<Response>;
}

pub struct Fetcher;

#[async_trait]
impl Fetch for Fetcher {
    async fn fetch(&self, url: &repo_url::RepoUrl) -> Result<Response> {
        Page::from(url).fetch().await
    }
}
