use std::fs;

use super::Fixtures;

mod topic_references {
    use super::*;

    #[test]
    fn no_leaks() {
        let f = Fixtures::copy("simple");
        let other = f.git.root.inner.join("other");
        fs::create_dir_all(&other).expect("unable create other repo");

        assert!(f.no_leaks().unwrap());

        let filename = other.join("file.txt");
        fs::write(&filename, "path: /wiki/some-path").expect("unable to write file");

        assert!(!f.no_leaks().unwrap());
    }
}
