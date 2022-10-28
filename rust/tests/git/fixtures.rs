use digraph::git::{
    Client, DataRoot, FetchTopicLiveSearch, FetchTopicLiveSearchResult, IndexMode, Mutation,
    OnMatchingSynonym, RepoLink, RepoTopic, Search, UpsertLink, UpsertLinkResult, UpsertTopic,
    UpsertTopicResult,
};
use digraph::http::{Fetch, Response};
use digraph::prelude::*;
use digraph::redis;
use digraph::types::Timespec;
use fs_extra::dir;
use scraper::html::Html;
use std::env;
use std::path::PathBuf;
use tempfile::{self, TempDir};

use super::{actor, parse_id};

struct Fetcher(String);

impl Fetch for Fetcher {
    fn fetch(&self, url: &RepoUrl) -> Result<Response> {
        Ok(Response {
            url: url.to_owned(),
            body: Html::parse_document(&self.0),
        })
    }
}

pub struct Fixtures {
    _tempdir: TempDir,
    pub git: Client,
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
        let git = Client::new(&actor(), &root, Timespec);

        Fixtures {
            _tempdir: tempdir,
            git,
            path,
            source,
        }
    }

    pub fn mutation(&self) -> Mutation {
        self.git.mutation(IndexMode::Update).unwrap()
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

    pub fn fetch_link<F>(&self, repo_id: &RepoId, link_id: &Oid, block: F)
    where
        F: Fn(RepoLink),
    {
        let link = self
            .git
            .fetch_link(repo_id, link_id)
            .unwrap_or_else(|| panic!("expected a link: {:?}", link_id));

        block(link);
    }

    pub fn fetch_topic<F>(&self, repo: &RepoId, topic_id: &Oid, block: F)
    where
        F: Fn(RepoTopic),
    {
        let topic = self
            .git
            .fetch_topic(repo, topic_id)
            .unwrap_or_else(|| panic!("expected a topic: {:?}", topic_id));

        block(topic);
    }

    pub fn leaked_data(&self) -> Result<Vec<(RepoId, String)>> {
        self.git.leaked_data()
    }

    pub fn topic(&self, repo: &RepoId, topic_id: &str) -> RepoTopic {
        let topic_id = Oid::try_from(topic_id).unwrap();
        self.git.fetch_topic(repo, &topic_id).unwrap()
    }

    pub fn find_topic(&self, name: &str) -> Option<Oid> {
        let FetchTopicLiveSearchResult {
            synonym_matches: matches,
            ..
        } = FetchTopicLiveSearch {
            limit: 10,
            repos: RepoIds::from(vec![RepoId::wiki()]),
            search: Search::parse(name).unwrap(),
            viewer: actor(),
        }
        .call(&self.git)
        .unwrap();

        let row = matches.iter().find(|row| row.name == name);

        row.map(|m| m.id.to_owned())
    }

    fn write(&self) {
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
        repo_id: &RepoId,
        url: &RepoUrl,
        title: Option<String>,
        add_parent_topic_id: Option<Oid>,
    ) -> UpsertLinkResult {
        let html = match &title {
            Some(title) => format!("<title>{}</title>", title),
            None => "<title>Some title</title>".into(),
        };

        let request = UpsertLink {
            actor: actor(),
            add_parent_topic_id,
            fetcher: Box::new(Fetcher(html)),
            repo_id: repo_id.to_owned(),
            url: url.normalized.to_owned(),
            title,
        };

        request
            .call(self.mutation(), &redis::Noop)
            .expect("expected a link")
    }

    pub fn upsert_topic(
        &self,
        repo: &RepoId,
        name: &str,
        parent_topic: &Oid,
        on_matching_synonym: OnMatchingSynonym,
    ) -> Result<UpsertTopicResult> {
        UpsertTopic {
            actor: actor(),
            parent_topic: parent_topic.to_owned(),
            locale: Locale::EN,
            name: name.into(),
            on_matching_synonym,
            repo_id: repo.to_owned(),
        }
        .call(self.mutation(), &redis::Noop)
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
        let root = Oid::root_topic();
        let repo = RepoId::wiki();

        let topic_path =
            parse_id("dPqrU4sZaPkNZEDyr9T68G4RJYV8bncmIXumedBNls9F994v8poSbxTo7dKK3Vhi");
        let UpsertTopicResult { repo_topic, .. } = UpsertTopic {
            actor: actor(),
            locale: Locale::EN,
            name: "Climate change".to_owned(),
            repo_id: repo.to_owned(),
            on_matching_synonym: OnMatchingSynonym::Update(topic_path),
            parent_topic: root.clone(),
        }
        .call(f.mutation(), &redis::Noop)
        .unwrap();
        let climate_change = repo_topic.unwrap();

        let url = RepoUrl::parse("https://en.wikipedia.org/wiki/Climate_change").unwrap();
        let result = f.upsert_link(
            &repo,
            &url,
            Some("Climate change".into()),
            Some(climate_change.topic_id().to_owned()),
        );
        println!("result: {:?}", result.link);

        let topic_id = parse_id("wxy3RN6zm8BJKr6kawH3ekvYwwYT5EEgIhm5nrRD69qm7audRylxmZSNY39Aa1Gj");
        let UpsertTopicResult { repo_topic, .. } = UpsertTopic {
            actor: actor(),
            locale: Locale::EN,
            name: "Weather".to_owned(),
            repo_id: RepoId::wiki(),
            on_matching_synonym: OnMatchingSynonym::Update(topic_id),
            parent_topic: root,
        }
        .call(f.mutation(), &redis::Noop)
        .unwrap();
        let weather = repo_topic.unwrap();

        let url = RepoUrl::parse("https://en.wikipedia.org/wiki/Weather").unwrap();
        f.upsert_link(
            &repo,
            &url,
            Some("Weather".into()),
            Some(weather.topic_id().to_owned()),
        );

        let topic_id = parse_id("F7EddRg9OPuLuk2oRMlO0Sm1v4OxgxQvzB3mRZxGfrqQ9dXjD4QKD6wuxOxucP13");
        let UpsertTopicResult { repo_topic, .. } = UpsertTopic {
            actor: actor(),
            locale: Locale::EN,
            name: "Climate change and weather".to_owned(),
            repo_id: repo.to_owned(),
            on_matching_synonym: OnMatchingSynonym::Update(topic_id),
            parent_topic: climate_change.topic_id().to_owned(),
        }
        .call(f.mutation(), &redis::Noop)
        .unwrap();
        let climate_change_weather = repo_topic.unwrap();

        UpdateTopicParentTopics {
            actor: actor(),
            repo_id: repo.to_owned(),
            parent_topic_ids: BTreeSet::from([
                climate_change.topic_id().to_owned(),
                weather.topic_id().to_owned(),
            ]),
            topic_id: climate_change_weather.topic_id().to_owned(),
        }
        .call(f.mutation(), &redis::Noop)
        .unwrap();

        let url =
            RepoUrl::parse("https://royalsociety.org/topics-policy/projects/climate-change-evidence-causes/question-13/")
                .unwrap();
        f.upsert_link(
            &repo,
            &url,
            Some("13. How does climate change affect the strength and frequency of floods, droughts, hurricanes, and tornadoes?".into()),
            Some(climate_change_weather.topic_id().to_owned()),
        );

        let url =
            RepoUrl::parse("https://climate.nasa.gov/resources/global-warming-vs-climate-change/")
                .unwrap();
        f.upsert_link(
            &repo,
            &url,
            Some("Overview: Weather, Global Warming, and Climate Change".into()),
            Some(climate_change_weather.topic_id().to_owned()),
        );

        f.write();
    }
}
