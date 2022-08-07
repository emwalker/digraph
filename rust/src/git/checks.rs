use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use super::Client;
use crate::prelude::*;
use crate::types::RepoPrefix;

pub struct LeakedData;

impl LeakedData {
    pub fn call(&self, client: &Client) -> Result<Vec<(RepoPrefix, String)>> {
        let root = client.root.path.to_owned();
        let repos = self.repos(&root)?;
        env::set_current_dir(&root)?;

        let mut leaks = vec![];
        for repo in repos {
            let output = Command::new("grep")
                .arg("-rl")
                .arg(repo.to_string())
                .arg(".")
                .output()?;
            let pat = format!(".{}", repo);

            let result = String::from_utf8(output.stdout)?;
            for line in result.lines() {
                if !line.starts_with(&pat) {
                    leaks.push((repo.to_owned(), line.to_owned()));
                }
            }
        }

        Ok(leaks)
    }

    fn repos(&self, root: &PathBuf) -> Result<Vec<RepoPrefix>> {
        let paths = fs::read_dir(&root)?;

        let mut repos = vec![];
        for path in paths {
            let path = path?.path();

            let filename = match &path.file_name() {
                Some(filename) => match filename.to_str() {
                    Some(filename) => filename,
                    None => {
                        log::error!("expected a filename: {:?}", path);
                        continue;
                    }
                },
                None => {
                    log::error!("expected a filename: {:?}", path);
                    continue;
                }
            };

            let repo = filename.to_string();
            repos.push(RepoPrefix::from_name(&repo)?);
        }

        Ok(repos)
    }
}
