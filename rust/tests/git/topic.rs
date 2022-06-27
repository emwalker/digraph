use super::{upsert_topic, Fixtures};
use digraph::git::{OnMatchingSynonym, Search};
use digraph::prelude::*;

#[actix_web::test]
async fn topic_is_added() {
    let f = Fixtures::copy("simple");
    let search = Search::parse("topic name").unwrap();

    let result = upsert_topic(
        &f,
        "Topic name",
        RepoPath::from("/wiki/00001"),
        OnMatchingSynonym::Ask,
    )
    .unwrap();

    assert!(result.saved);
    assert_eq!(result.matching_synonyms, &[]);

    let topic = &result.topic;
    assert!(topic.is_some());

    let topic = (*topic).clone().unwrap();
    let path = RepoPath::from(&topic.metadata.path);
    assert!(f.repo.appears_in(&search, &path).unwrap());
}

#[actix_web::test]
async fn action_requested() {
    let f = Fixtures::copy("simple");

    let result = upsert_topic(
        &f,
        "Topic name",
        RepoPath::from("/wiki/00001"),
        OnMatchingSynonym::Ask,
    )
    .unwrap();
    assert!(result.saved);

    let result = upsert_topic(
        &f,
        "Topic Name",
        RepoPath::from("/wiki/00001"),
        OnMatchingSynonym::Ask,
    )
    .unwrap();

    assert!(result.topic.is_none());
    assert!(!result.saved);
    assert_ne!(result.matching_synonyms, &[]);
}

#[actix_web::test]
async fn update_topic() {
    let f = Fixtures::copy("simple");

    let result = upsert_topic(
        &f,
        "Topic name",
        RepoPath::from("/wiki/00001"),
        OnMatchingSynonym::Ask,
    )
    .unwrap();
    assert!(result.saved);
    let path = RepoPath::from(&result.topic.unwrap().metadata.path);

    let result = upsert_topic(
        &f,
        "Topic Name",
        RepoPath::from("/wiki/00002"),
        OnMatchingSynonym::Update(path),
    )
    .unwrap();

    assert!(result.topic.is_some());
    assert!(result.saved);

    let parent_topics = result
        .topic
        .unwrap()
        .parent_topics
        .iter()
        .map(|topic| topic.path.to_owned())
        .collect::<Vec<String>>();

    assert_eq!(parent_topics, &["/wiki/00001", "/wiki/00002"]);
}

#[actix_web::test]
async fn create_distinct() {
    let f = Fixtures::copy("simple");

    let result = upsert_topic(
        &f,
        "Topic name",
        RepoPath::from("/wiki/00001"),
        OnMatchingSynonym::Ask,
    )
    .unwrap();
    assert!(result.saved);
    let path1 = &result.topic.unwrap().metadata.path;

    let result = upsert_topic(
        &f,
        "Topic Name",
        RepoPath::from("/wiki/00002"),
        OnMatchingSynonym::CreateDistinct,
    )
    .unwrap();

    assert!(result.topic.is_some());
    assert!(result.saved);
    let path2 = &result.topic.unwrap().metadata.path;

    assert_ne!(path1, path2);

    let matches = f.repo.git.synonym_matches("/wiki", "Topic name").unwrap();
    let mut names = matches
        .iter()
        .map(|m| m.entry.name.to_owned())
        .collect::<Vec<String>>();
    names.sort();
    assert_eq!(names, vec!["Topic Name", "Topic name"]);
}

#[actix_web::test]
async fn parent_topic_updated() {
    let f = Fixtures::copy("simple");
    let parent = f.repo.git.fetch_topic("/wiki/00001").unwrap();
    assert_eq!(parent.children, vec![]);

    let result = upsert_topic(
        &f,
        "Topic name",
        RepoPath::from("/wiki/00001"),
        OnMatchingSynonym::Ask,
    )
    .unwrap();
    assert!(result.saved);
    let child_path = &result.topic.unwrap().metadata.path;

    let parent = f.repo.git.fetch_topic("/wiki/00001").unwrap();
    let children = parent
        .children
        .iter()
        .map(|child| child.path.to_owned())
        .collect::<Vec<String>>();

    assert_eq!(children, vec![child_path.to_string()]);
}
