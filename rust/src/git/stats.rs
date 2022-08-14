use actix_rt;
use serde::{Deserialize, Serialize};

use super::{core, Client};
use crate::prelude::*;

async fn compute_stats<C>(
    view: core::View,
    prefix: RepoPrefix,
    commit: String,
    cache: C,
) -> Result<()>
where
    C: CacheStats + Clone + std::fmt::Debug + Send + 'static,
{
    let key = format!("{}:{}", prefix, commit);
    log::info!("computing stats for {} ...", key);
    let stats = view.stats()?;
    cache.save(&prefix, &commit, &stats, None)?;
    log::info!("stats for {} saved to cache", key);
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RepoStats {
    pub computing: bool,
    pub link_count: Option<usize>,
    pub topic_count: Option<usize>,
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
        self.stats.iter().map(|s| s.link_count.unwrap_or(0)).sum()
    }

    pub fn topic_count(&self) -> usize {
        self.stats.iter().map(|s| s.topic_count.unwrap_or(0)).sum()
    }
}

pub trait CacheStats {
    fn fetch(&self, repo: &RepoPrefix, oid: &str) -> Result<Option<RepoStats>>;

    fn save(&self, repo: &RepoPrefix, oid: &str, stats: &RepoStats, ttl: Option<u32>)
        -> Result<()>;
}

pub struct FetchStats {
    pub viewer: Viewer,
}

pub struct FetchStatsResult {
    pub stats: Stats,
}

impl FetchStats {
    pub async fn call<C>(&self, git: &Client, cache: C) -> Result<FetchStatsResult>
    where
        C: CacheStats + Clone + Send + std::fmt::Debug + 'static,
    {
        let mut stats = vec![];

        for prefix in self.viewer.read_repos.iter() {
            let view = git.view(prefix)?;

            let commit = view.commit.to_string();
            let s = match cache.fetch(prefix, &commit)? {
                Some(stats) => stats,

                None => {
                    let stats = RepoStats {
                        computing: true,
                        link_count: None,
                        topic_count: None,
                    };
                    // Save a placeholder that will be updated with the computed values.  Expires
                    // after 180 seconds in case something happens and it should be retried.
                    cache.save(prefix, &commit, &stats, Some(120))?;

                    let view = view.duplicate()?;
                    let background_task =
                        compute_stats(view, prefix.to_owned(), commit.to_owned(), cache.to_owned());
                    actix_rt::spawn(background_task);

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
