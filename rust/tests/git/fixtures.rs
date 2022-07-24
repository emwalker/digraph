use async_trait::async_trait;
use digraph::git::{
    DataRoot, FetchTopicLiveSearch, FetchTopicLiveSearchResult, Git, Link, OnMatchingSynonym,
    Search, Topic, UpsertLink, UpsertLinkResult, UpsertTopic, UpsertTopicResult,
};
use digraph::http::{Fetch, Response};
use digraph::prelude::*;
use digraph::redis;
use fs_extra::dir;
use futures::executor::block_on;
use scraper::html::Html;
use std::env;
use std::path::PathBuf;
use tempfile::{self, TempDir};

use super::actor;

struct Fetcher(String);

#[async_trait]
impl Fetch for Fetcher {
    async fn fetch(&self, url: &RepoUrl) -> Result<Response> {
        Ok(Response {
            url: url.to_owned(),
            body: Html::parse_document(&self.0),
        })
    }
}

pub struct Fixtures {
    _tempdir: TempDir,
    pub git: Git,
    pub path: PathBuf,
    source: PathBuf,
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
        let git = Git::new(&Viewer::super_user(), &root);

        Fixtures {
            _tempdir: tempdir,
            git,
            path,
            source,
        }
    }

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

    pub fn no_leaks(&self) -> Result<bool> {
        Ok(self.leaked_data()?.is_empty())
    }

    pub fn fetch_link<F>(&self, path: &RepoPath, block: F)
    where
        F: Fn(Link),
    {
        let link = self.git.fetch_link(path).unwrap();
        block(link);
    }

    pub fn fetch_topic<F>(&self, path: &RepoPath, block: F)
    where
        F: Fn(Topic),
    {
        let topic = self.git.fetch_topic(path).unwrap();
        block(topic);
    }

    pub fn leaked_data(&self) -> Result<Vec<(RepoPrefix, String)>> {
        self.git.leaked_data()
    }

    pub fn topic_path(&self, name: &str) -> Result<Option<RepoPath>> {
        let FetchTopicLiveSearchResult {
            synonym_matches: matches,
            ..
        } = FetchTopicLiveSearch {
            limit: 10,
            prefixes: vec![RepoPrefix::wiki()],
            search: Search::parse(name).unwrap(),
            viewer: actor(),
        }
        .call(&self.git)
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

    pub fn upsert_link(
        &self,
        repo: &RepoPrefix,
        url: &RepoUrl,
        title: Option<String>,
        parent_topic: Option<String>,
    ) -> UpsertLinkResult {
        let html = match &title {
            Some(title) => format!("<title>{}</title>", title),
            None => "<title>Some title</title>".into(),
        };

        let add_parent_topic_path = parent_topic.as_ref().map(RepoPath::from);

        let request = UpsertLink {
            actor: actor(),
            add_parent_topic_path,
            fetcher: Box::new(Fetcher(html)),
            repo: repo.to_owned(),
            url: url.normalized.to_owned(),
            title,
        };

        block_on(request.call(&self.git, &redis::Noop)).unwrap()
    }

    pub fn upsert_topic(
        &self,
        repo: &RepoPrefix,
        name: &str,
        parent_topic: &RepoPath,
        on_matching_synonym: OnMatchingSynonym,
    ) -> Result<UpsertTopicResult> {
        UpsertTopic {
            actor: actor(),
            parent_topic: parent_topic.to_owned(),
            locale: Locale::EN,
            name: name.into(),
            on_matching_synonym,
            repo: repo.to_owned(),
        }
        .call(&self.git, &redis::Noop)
    }
}

mod tests {
    use super::*;

    use digraph::git::{UpdateTopicParentTopics, UpsertTopic, UpsertTopicResult};
    use std::collections::BTreeSet;

    // #[actix_web::test]
    #[allow(dead_code)]
    fn update_simple_fixtures() {
        let f = Fixtures::copy("simple");
        let root = RepoPath::from(WIKI_ROOT_TOPIC_PATH);

        let path = RepoPath::from(
            "/wiki/dPqrU4sZaPkNZEDyr9T68G4RJYV8bncmIXumedBNls9F994v8poSbxTo7dKK3Vhi",
        );
        let UpsertTopicResult { topic, .. } = UpsertTopic {
            actor: actor(),
            locale: Locale::EN,
            name: "Climate change".to_owned(),
            repo: RepoPrefix::wiki(),
            on_matching_synonym: OnMatchingSynonym::Update(path),
            parent_topic: root.clone(),
        }
        .call(&f.git, &redis::Noop)
        .unwrap();
        let climate_change = topic.unwrap().path();

        let url = RepoUrl::parse("https://en.wikipedia.org/wiki/Climate_change").unwrap();
        let result = f.upsert_link(
            &RepoPrefix::wiki(),
            &url,
            Some("Climate change".into()),
            Some(climate_change.inner.to_owned()),
        );
        println!("result: {:?}", result.link);

        let path = RepoPath::from(
            "/wiki/wxy3RN6zm8BJKr6kawH3ekvYwwYT5EEgIhm5nrRD69qm7audRylxmZSNY39Aa1Gj",
        );
        let UpsertTopicResult { topic, .. } = UpsertTopic {
            actor: actor(),
            locale: Locale::EN,
            name: "Weather".to_owned(),
            repo: RepoPrefix::wiki(),
            on_matching_synonym: OnMatchingSynonym::Update(path),
            parent_topic: root.clone(),
        }
        .call(&f.git, &redis::Noop)
        .unwrap();
        let weather = topic.unwrap().path();

        let url = RepoUrl::parse("https://en.wikipedia.org/wiki/Weather").unwrap();
        f.upsert_link(
            &RepoPrefix::wiki(),
            &url,
            Some("Weather".into()),
            Some(weather.inner.to_owned()),
        );

        let path = RepoPath::from(
            "/wiki/F7EddRg9OPuLuk2oRMlO0Sm1v4OxgxQvzB3mRZxGfrqQ9dXjD4QKD6wuxOxucP13",
        );
        let UpsertTopicResult { topic, .. } = UpsertTopic {
            actor: actor(),
            locale: Locale::EN,
            name: "Climate change and weather".to_owned(),
            repo: RepoPrefix::wiki(),
            on_matching_synonym: OnMatchingSynonym::Update(path),
            parent_topic: climate_change.clone(),
        }
        .call(&f.git, &redis::Noop)
        .unwrap();
        let climate_change_weather = topic.unwrap().path();

        UpdateTopicParentTopics {
            actor: actor(),
            parent_topic_paths: BTreeSet::from([climate_change.clone(), weather.clone()]),
            topic_path: climate_change_weather.clone(),
        }
        .call(&f.git, &redis::Noop)
        .unwrap();

        let url =
            RepoUrl::parse("https://royalsociety.org/topics-policy/projects/climate-change-evidence-causes/question-13/")
                .unwrap();
        f.upsert_link(
            &RepoPrefix::wiki(),
            &url,
            Some("13. How does climate change affect the strength and frequency of floods, droughts, hurricanes, and tornadoes?".into()),
            Some(climate_change_weather.inner.to_owned()),
        );

        let url =
            RepoUrl::parse("https://climate.nasa.gov/resources/global-warming-vs-climate-change/")
                .unwrap();
        f.upsert_link(
            &RepoPrefix::wiki(),
            &url,
            Some("Overview: Weather, Global Warming, and Climate Change".into()),
            Some(climate_change_weather.inner.to_owned()),
        );

        f.update();
    }
}
