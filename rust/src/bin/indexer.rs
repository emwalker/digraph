use digraph::config;
use digraph::git;
use digraph::prelude::*;
use digraph::types::{Timespec, Viewer};
use digraph::typesense as ts;

fn main() {
    let config = config::Config::load().unwrap();
    env_logger::init();

    let dirname = config.digraph_data_directory;
    let repo_id = RepoId::wiki();
    let topic_id = ExternalId::root_topic();
    let root = git::DataRoot::new(dirname.into());
    let view = git::Client::new(&Viewer::service_account(), &root, Timespec)
        .view(&repo_id)
        .expect("failed to get view");
    let iter = ts::GenerateRecords::new(&view, topic_id, Locale::EN);

    for record in iter {
        let s = serde_json::to_string(&record).expect("failed to serialize record");
        println!("{s}");
    }
}
