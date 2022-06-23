use async_graphql::dataloader::*;
use std::collections::HashMap;

use crate::git;
use crate::prelude::*;
use crate::schema::{Link, WIKI_REPOSITORY_ID};

impl From<&git::Link> for Link {
    fn from(link: &git::Link) -> Self {
        let meta = &link.metadata;
        let parent_topic_paths = link
            .parent_topics
            .iter()
            .map(|topic| RepoPath::from(&topic.path))
            .collect::<Vec<RepoPath>>();

        Self {
            path: RepoPath::from(&meta.path),
            newly_added: false,
            parent_topic_paths,
            repository_id: WIKI_REPOSITORY_ID.into(),
            viewer_review: None,
            title: meta.title.clone(),
            url: meta.url.clone(),
        }
    }
}

pub struct LinkLoader {
    viewer: Viewer,
    git: git::Git,
}

impl LinkLoader {
    pub fn new(viewer: Viewer, git: git::Git) -> Self {
        Self { viewer, git }
    }
}

#[async_trait::async_trait]
impl Loader<String> for LinkLoader {
    type Value = Link;
    type Error = Error;

    async fn load(&self, paths: &[String]) -> Result<HashMap<String, Self::Value>> {
        log::debug!("batch links: {:?}", paths);
        let mut map: HashMap<_, _> = HashMap::new();

        for path in paths {
            let link = match &self.git.get(path)? {
                git::Object::Link(link) => Link::from(link),
                other => {
                    return Err(Error::Repo(format!("expected a link: {:?}", other)));
                }
            };
            map.insert(path.to_owned(), link);
        }

        Ok(map)
    }
}
