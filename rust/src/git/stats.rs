use serde::{Deserialize, Serialize};

use super::Client;
use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct RepoStats {
    pub link_count: usize,
    pub topic_count: usize,
}

impl TryInto<RepoStats> for String {
    type Error = Error;

    fn try_into(self) -> Result<RepoStats> {
        Ok(serde_yaml::from_slice(self.as_bytes())?)
    }
}

impl TryInto<String> for &RepoStats {
    type Error = Error;

    fn try_into(self) -> Result<String> {
        Ok(serde_json::to_string(self)?)
    }
}

#[derive(Debug)]
pub struct Stats {
    pub stats: Vec<RepoStats>,
}

impl Stats {
    pub fn link_count(&self) -> usize {
        self.stats.iter().map(|s| s.link_count).sum()
    }

    pub fn topic_count(&self) -> usize {
        self.stats.iter().map(|s| s.topic_count).sum()
    }
}

pub trait CacheStats {
    fn fetch(&self, repo: &RepoPrefix, oid: &str) -> Result<Option<RepoStats>>;

    fn save(&self, repo: &RepoPrefix, oid: &str, stats: &RepoStats) -> Result<()>;
}

pub struct FetchStats {
    pub viewer: Viewer,
}

pub struct FetchStatsResult {
    pub stats: Stats,
}

impl FetchStats {
    pub fn call<C>(&self, git: &Client, cache: &C) -> Result<FetchStatsResult>
    where
        C: CacheStats,
    {
        let mut stats = vec![];

        for prefix in self.viewer.read_repos.iter() {
            let view = git.view(prefix)?;

            let commit = view.commit.to_string();
            let s = match cache.fetch(prefix, &commit)? {
                Some(stats) => stats,

                None => {
                    let stats = view.stats()?;
                    cache.save(prefix, &commit, &stats)?;
                    stats
                }
            };

            stats.push(s);
        }

        Ok(FetchStatsResult {
            stats: Stats { stats },
        })
    }
}
