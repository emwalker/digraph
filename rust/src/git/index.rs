use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;
use std::path::PathBuf;
use unidecode::unidecode;

use super::{Git, API_VERSION};
use crate::prelude::*;

const SPECIAL_CHARS: &[char] = &[
    '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<', '=',
    '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~',
];

#[derive(Debug, Deserialize, Serialize)]
struct BTreeIndex {
    api_version: String,
    paths: BTreeMap<String, BTreeSet<String>>,
}

pub struct Phrase {
    pub exact: bool,
    pub tokens: Vec<String>,
}

impl Phrase {
    pub fn approximate(phrase: &str) -> Self {
        let phrase = unidecode(phrase).replace(SPECIAL_CHARS, "").to_lowercase();
        let tokens = phrase
            .split_whitespace()
            .filter_map(|s| {
                if s.len() >= 3 && s.len() <= 20 {
                    Some(s.to_string())
                } else {
                    None
                }
            })
            .collect_vec();

        Self {
            exact: false,
            tokens,
        }
    }

    pub fn lowercase(phrase: &str) -> Self {
        Self {
            exact: true,
            tokens: vec![phrase.to_lowercase()],
        }
    }
}

#[derive(Debug)]
pub struct PathIndex {
    filename: PathBuf,
    index: BTreeIndex,
}

impl PathIndex {
    pub fn load(filename: &PathBuf) -> Result<Self> {
        let index = if filename.as_path().exists() {
            let fh = std::fs::File::open(&filename).map_err(|e| Error::Repo(format!("{:?}", e)))?;
            serde_yaml::from_reader(fh)?
        } else {
            BTreeIndex {
                api_version: API_VERSION.to_owned(),
                paths: BTreeMap::new(),
            }
        };

        Ok(Self {
            filename: filename.to_owned(),
            index,
        })
    }

    pub fn index(&mut self, path: &RepoPath, token: &str) -> Result<()> {
        let paths = self
            .index
            .paths
            .entry(token.to_owned())
            .or_insert(BTreeSet::new());
        paths.insert(path.to_string());
        Ok(())
    }

    pub fn indexed_on(&self, path: &RepoPath, token: &str) -> Result<bool> {
        Ok(match &self.index.paths.get(token) {
            Some(paths) => paths.contains(&path.inner),
            None => false,
        })
    }

    pub fn write(&self) -> Result<()> {
        let dest = self.filename.parent().expect("expected a parent directory");
        fs::create_dir_all(&dest).ok();
        let s = serde_yaml::to_string(&self.index)?;
        log::debug!("saving {:?}", self.filename);
        fs::write(&self.filename, s).map_err(Error::from)
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct IndexKey {
    pub prefix: String,
    pub field: String,
}

pub struct Indexer {
    git: Git,
    indexes: HashMap<IndexKey, PathIndex>,
}

impl Indexer {
    pub fn new(git: &Git) -> Self {
        Self {
            git: git.clone(),
            indexes: HashMap::new(),
        }
    }

    pub fn save(&self) -> Result<()> {
        for index in self.indexes.values() {
            index.write()?;
        }
        Ok(())
    }

    pub fn index(&mut self, path: &RepoPath, phrases: &[Phrase]) -> Result<()> {
        for phrase in phrases {
            for token in &phrase.tokens {
                let key = self.git.index_key(path, token)?;
                let index = self
                    .indexes
                    .entry(key.to_owned())
                    .or_insert(self.git.index_for(&key)?);
                index.index(path, token)?;
            }
        }
        Ok(())
    }
}

impl std::fmt::Debug for Indexer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Indexer")
            .field("indexes", &self.indexes.keys())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn punctuation() {
        let phrase = Phrase::approximate("one.!?:`#$@*&;+-{}[]()/\\'\",=    <> two");
        assert_eq!(phrase.tokens, &["one", "two"]);
    }

    #[test]
    fn uppercase_letters() {
        let phrase = Phrase::approximate("One TWO three");
        assert_eq!(phrase.tokens, &["one", "two", "three"]);
    }

    #[test]
    fn unicode_characters() {
        let phrase = Phrase::approximate("Æneid étude 北亰 ᔕᓇᓇ げんまい茶");
        assert_eq!(
            phrase.tokens,
            &["aeneid", "etude", "bei", "jing", "shanana", "genmaicha"]
        );
    }

    #[test]
    fn token_length() {
        let token = (0..=20).map(|_| "a").collect::<String>();
        assert_eq!(token.len(), 21);

        let phrase = Phrase::approximate(&format!("a aa aaa aaaa {}", token));
        assert_eq!(phrase.tokens, &["aaa", "aaaa"]);
    }
}
