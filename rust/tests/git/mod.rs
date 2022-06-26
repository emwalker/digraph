use fs_extra::dir;
use std::env;
use std::path::PathBuf;
use tempfile::{self, TempDir};

use digraph::git::{DataRoot, Git, Repository};
use digraph::prelude::*;

struct Fixtures {
    path: PathBuf,
    source: PathBuf,
    repo: Repository,
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
        let repo = Repository::new("/wiki", git);

        Fixtures {
            _tempdir: tempdir,
            path,
            repo,
            source,
        }
    }
}

impl Fixtures {
    fn copy(fixture_dirname: &str) -> Self {
        let fixture = Fixtures::blank(fixture_dirname);
        let options = dir::CopyOptions {
            overwrite: true,
            ..Default::default()
        };
        log::debug!("copying: {:?}", fixture.source);
        dir::copy(&fixture.source, &fixture.path, &options).unwrap_or_else(|_| {
            panic!("problem copying {:?} to {:?}", fixture.source, fixture.path)
        });
        fixture
    }
}

fn actor() -> Viewer {
    Viewer {
        user_id: "2".into(),
        query_ids: vec!["2".into()],
        mutation_ids: vec!["2".into()],
        session_id: Some("2".into()),
    }
}

mod link;
