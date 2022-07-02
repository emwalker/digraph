use async_graphql::dataloader::*;
use std::collections::HashMap;

use crate::git;
use crate::prelude::*;

#[allow(dead_code)]
pub struct ObjectLoader {
    viewer: Viewer,
    git: git::Git,
}

impl ObjectLoader {
    pub fn new(viewer: Viewer, git: git::Git) -> Self {
        Self { viewer, git }
    }
}

#[async_trait::async_trait]
impl Loader<String> for ObjectLoader {
    type Value = git::Object;
    type Error = Error;

    async fn load(&self, paths: &[String]) -> Result<HashMap<String, Self::Value>> {
        log::debug!("batch load topics: {:?}", paths);
        let mut map: HashMap<_, _> = HashMap::new();

        for path in paths {
            let object = &self.git.fetch(path)?;
            map.insert(path.to_owned(), object.clone());
        }

        Ok(map)
    }
}
