use crate::prelude::*;

mod page;
pub use page::*;
mod repo_url;
pub use repo_url::*;

pub trait Fetch {
    fn fetch(&self, url: &repo_url::RepoUrl) -> Result<Response>;
}

pub struct Fetcher;

impl Fetch for Fetcher {
    fn fetch(&self, url: &repo_url::RepoUrl) -> Result<Response> {
        Page::from(url).fetch()
    }
}
