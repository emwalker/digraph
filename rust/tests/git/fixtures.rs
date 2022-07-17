use digraph::git::{
    DataRoot, FetchTopicLiveSearch, FetchTopicLiveSearchResult, Git, OnMatchingSynonym, Repository,
    Search,
};
use digraph::prelude::*;
use digraph::redis;
use fs_extra::dir;
use std::env;
use std::path::PathBuf;
use tempfile::{self, TempDir};

use super::{actor, upsert_link};

pub struct Fixtures {
    path: PathBuf,
    source: PathBuf,
    pub repo: Repository,
    _tempdir: TempDir,
}

impl Fixtures {
    fn blank(fixture_dirname: &str) -> Self {
        let root = &env::var("CARGO_MANIFEST_DIR").expect("$CARGO_MANIFEST_DIR");
        let mut source = PathBuf::from(root);
        source.push("tests/fixtures");
        source.push(&fixture_dirname);

        let tempdir = tempfile::tempdir().unwrap();
        let path = PathBuf::from(&tempdir.path());
        let root = DataRoot::new(path.clone());
        let git = Git::new(root);
        let repo = Repository::new(&RepoPrefix::from("/wiki/"), git);

        Fixtures {
            _tempdir: tempdir,
            path,
            repo,
            source,
        }
    }
}

impl Fixtures {
    pub fn copy(fixture_dirname: &str) -> Self {
        let fixture = Fixtures::blank(fixture_dirname);
        let options = dir::CopyOptions {
            overwrite: true,
            content_only: true,
            ..Default::default()
        };
        log::debug!("copying: {:?}", fixture.source);
        dir::copy(&fixture.source, &fixture.path, &options).unwrap_or_else(|_| {
            panic!("problem copying {:?} to {:?}", fixture.source, fixture.path)
        });
        fixture
    }

    pub fn topic_path(&self, name: &str) -> Result<Option<RepoPath>> {
        let FetchTopicLiveSearchResult {
            synonym_matches: matches,
            ..
        } = FetchTopicLiveSearch {
            limit: 10,
            prefixes: vec![RepoPrefix::from("/wiki/")],
            search: Search::parse(name).unwrap(),
            viewer: actor(),
        }
        .call(&self.repo.git)
        .unwrap();

        let row = matches.iter().find(|row| row.name == name);

        Ok(row.map(|m| RepoPath::from(&m.path)))
    }

    fn update(&self) {
        let options = dir::CopyOptions {
            overwrite: true,
            content_only: true,
            ..Default::default()
        };
        dir::copy(&self.path, &self.source, &options)
            .unwrap_or_else(|_| panic!("problem updating {:?} to {:?}", self.path, self.source));
    }
}

mod tests {
    use super::*;

    use digraph::git::{UpdateTopicParentTopics, UpsertTopic, UpsertTopicResult};
    use std::collections::BTreeSet;

    // #[actix_web::test]
    #[allow(dead_code)]
    async fn update_simple_fixtures() {
        let f = Fixtures::copy("simple");
        let root = RepoPath::from(WIKI_ROOT_TOPIC_PATH);

        let path = RepoPath::from(
            "/wiki/dPqrU4sZaPkNZEDyr9T68G4RJYV8bncmIXumedBNls9F994v8poSbxTo7dKK3Vhi",
        );
        let UpsertTopicResult { topic, .. } = UpsertTopic {
            actor: actor(),
            locale: Locale::EN,
            name: "Climate change".to_owned(),
            prefix: RepoPrefix::from("/wiki/"),
            on_matching_synonym: OnMatchingSynonym::Update(path),
            parent_topic: root.clone(),
        }
        .call(&f.repo.git, &redis::Noop)
        .unwrap();
        let climate_change = topic.unwrap().path();

        let url = RepoUrl::parse("https://en.wikipedia.org/wiki/Climate_change").unwrap();
        upsert_link(
            &f,
            &url,
            Some("Climate change".into()),
            Some(climate_change.inner.to_owned()),
        )
        .await;

        let path = RepoPath::from(
            "/wiki/wxy3RN6zm8BJKr6kawH3ekvYwwYT5EEgIhm5nrRD69qm7audRylxmZSNY39Aa1Gj",
        );
        let UpsertTopicResult { topic, .. } = UpsertTopic {
            actor: actor(),
            locale: Locale::EN,
            name: "Weather".to_owned(),
            prefix: RepoPrefix::from("/wiki/"),
            on_matching_synonym: OnMatchingSynonym::Update(path),
            parent_topic: root.clone(),
        }
        .call(&f.repo.git, &redis::Noop)
        .unwrap();
        let weather = topic.unwrap().path();

        let url = RepoUrl::parse("https://en.wikipedia.org/wiki/Weather").unwrap();
        upsert_link(
            &f,
            &url,
            Some("Weather".into()),
            Some(weather.inner.to_owned()),
        )
        .await;

        let path = RepoPath::from(
            "/wiki/F7EddRg9OPuLuk2oRMlO0Sm1v4OxgxQvzB3mRZxGfrqQ9dXjD4QKD6wuxOxucP13",
        );
        let UpsertTopicResult { topic, .. } = UpsertTopic {
            actor: actor(),
            locale: Locale::EN,
            name: "Climate change and weather".to_owned(),
            prefix: RepoPrefix::from("/wiki/"),
            on_matching_synonym: OnMatchingSynonym::Update(path),
            parent_topic: climate_change.clone(),
        }
        .call(&f.repo.git, &redis::Noop)
        .unwrap();
        let climate_change_weather = topic.unwrap().path();

        UpdateTopicParentTopics {
            actor: actor(),
            parent_topic_paths: BTreeSet::from([climate_change.clone(), weather.clone()]),
            topic_path: climate_change_weather.clone(),
        }
        .call(&f.repo.git, &redis::Noop)
        .unwrap();

        let url =
            RepoUrl::parse("https://royalsociety.org/topics-policy/projects/climate-change-evidence-causes/question-13/")
                .unwrap();
        upsert_link(
            &f,
            &url,
            Some("13. How does climate change affect the strength and frequency of floods, droughts, hurricanes, and tornadoes?".into()),
            Some(climate_change_weather.inner.to_owned()),
        )
        .await;

        let url =
            RepoUrl::parse("https://climate.nasa.gov/resources/global-warming-vs-climate-change/")
                .unwrap();
        upsert_link(
            &f,
            &url,
            Some("Overview: Weather, Global Warming, and Climate Change".into()),
            Some(climate_change_weather.inner.to_owned()),
        )
        .await;

        f.update();
    }
}
