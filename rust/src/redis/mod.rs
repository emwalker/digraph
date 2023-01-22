use redis_rs::{self, Commands};
use std::collections::{BTreeSet, HashMap, HashSet};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::git;
use crate::prelude::*;
use crate::types::{Downset, TopicPath};

#[derive(Clone, Debug)]
pub struct Noop;

impl git::SaveChangesForPrefix for Noop {
    fn save(
        &self,
        _prefix: RepoId,
        _changes: &HashMap<RepoId, BTreeSet<git::activity::Change>>,
    ) -> Result<()> {
        // Do nothing
        Ok(())
    }
}

impl git::CacheStats for Noop {
    fn fetch(&self, _repo_id: RepoId, _commit: &str) -> Result<Option<git::RepoStats>> {
        Ok(None)
    }

    fn save(
        &self,
        _repo: RepoId,
        _commit: &str,
        _stats: &git::RepoStats,
        _expires: Option<u32>,
    ) -> Result<()> {
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Key(String);

impl Key {
    fn downset(path: &TopicPath) -> Self {
        Self(format!(
            "repo:{}:topic:{}:{}:down",
            path.repo_id, path.topic_id, path.topic_oid
        ))
    }
}

impl redis_rs::ToRedisArgs for Key {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis_rs::RedisWrite,
    {
        out.write_arg(self.0.as_bytes());
    }
}

#[derive(Clone, Debug)]
pub struct Redis {
    url: String,
}

impl git::CacheStats for Arc<Redis> {
    fn fetch(&self, repo_id: RepoId, commit: &str) -> Result<Option<git::RepoStats>> {
        let key = self.stats_key(repo_id, commit);
        let mut con = self.connection().unwrap();
        let s: Option<String> = redis::cmd("GET").arg(key).query(&mut con)?;

        match s {
            Some(s) => {
                let stats: git::RepoStats = s.try_into()?;
                Ok(Some(stats))
            }

            None => Ok(None),
        }
    }

    fn save(
        &self,
        repo_id: RepoId,
        commit: &str,
        stats: &git::RepoStats,
        ttl: Option<u32>,
    ) -> Result<()> {
        let key = self.stats_key(repo_id, commit);
        let mut con = self.connection().unwrap();
        let s: String = stats.try_into()?;

        let mut command = redis::cmd("SET");
        let mut command = command.arg(key).arg(s);
        if let Some(ttl) = ttl {
            command = command.arg("EX").arg(ttl);
        }
        command.query(&mut con)?;

        Ok(())
    }
}

impl Redis {
    pub fn new(url: String) -> Result<Self> {
        Ok(Redis { url })
    }

    // For each topic read path (a commit, oid, repo_id combo), fetch the downset for the repo
    // topic, save the oids in the downset to redis and perform the intersection of the sets,
    // returning the resulting list of oids.
    pub fn intersection<F>(
        &self,
        fetch: &F,
        topic_paths: &[TopicPath],
    ) -> Result<HashSet<ExternalId>>
    where
        F: Downset,
    {
        if topic_paths.is_empty() {
            log::warn!("no paths provided for transitive closure, exiting early");
            return Ok(HashSet::new());
        }

        log::info!("redis: fetching intersection of paths {:?}", topic_paths);
        let (head, tail) = topic_paths.split_at(1);
        let mut con = self.connection()?;
        let mut keys = vec![];

        match head.get(0) {
            Some(path) => {
                let key = Key::downset(path);
                keys.push(key.clone());

                if !con.exists(&key)? {
                    log::info!("redis: {:?} not found in redis, saving", key);
                    let set = fetch.downset(path);
                    self.save_downset(&mut con, &key, &set)?;

                    if set.is_empty() {
                        return Ok(HashSet::new());
                    }
                }

                for other_path in tail {
                    let key = Key::downset(other_path);
                    keys.push(key.clone());

                    if !con.exists(&key)? {
                        log::info!("redis: {:?} not found in redis, saving", key);
                        let set = fetch.downset(other_path);
                        self.save_downset(&mut con, &key, &set)?;

                        if set.is_empty() {
                            return Ok(HashSet::new());
                        }
                    }
                }

                let set: HashSet<String> = con.sinter(&keys)?;
                Ok(set
                    .iter()
                    .map(ExternalId::try_from)
                    .collect::<Result<HashSet<ExternalId>>>()?)
            }

            None => Ok(HashSet::new()),
        }
    }

    pub fn connection(&self) -> Result<redis_rs::Connection> {
        let client = redis_rs::Client::open(self.url.clone())?;
        Ok(client.get_connection()?)
    }

    // Since redis keys have the commit hash of an immutible Git commit, they do not need to have
    // an expiry.
    fn save_downset(
        &self,
        con: &mut redis_rs::Connection,
        key: &Key,
        set: &HashSet<ExternalId>,
    ) -> Result<()> {
        redis_rs::transaction(con, &[key], |con, pipe| {
            let set = set
                .iter()
                .map(ExternalId::to_string)
                .collect::<HashSet<String>>();
            if set.is_empty() {
                pipe.del(key).ignore()
            } else {
                pipe.sadd(key, set).ignore()
            }
            .query(con)
        })?;

        Ok(())
    }

    fn stats_key(&self, repo_id: RepoId, commit: &str) -> String {
        format!("stats:{}:{}", repo_id, commit)
    }
}

impl git::SaveChangesForPrefix for Arc<Redis> {
    fn save(
        &self,
        repo_id: RepoId,
        repo_changes: &HashMap<RepoId, BTreeSet<git::activity::Change>>,
    ) -> Result<()> {
        let mut con = self.connection()?;
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let empty = BTreeSet::new();
        let changes = repo_changes.get(&repo_id).unwrap_or(&empty);

        let mut args = vec![];
        for change in changes {
            let string = serde_yaml::to_string(change)?;
            args.push((now, string));
        }

        let key = Key(format!("activity:{}", repo_id));
        if !args.is_empty() {
            log::info!("saving changes to {:?}", key);
            con.zadd_multiple(key, &args)?;
        } else {
            log::info!("no changes to save to {:?} key, skipping", key);
        }

        Ok(())
    }
}

impl git::activity::ActivityForPrefix for Arc<Redis> {
    fn fetch_activity(&self, repo_id: RepoId, first: usize) -> Result<Vec<git::activity::Change>> {
        let key = Key(format!("activity:{}", repo_id));
        log::info!("fetching activity for prefix {:?} from Redis", key);
        let mut con = self.connection()?;

        let iter: redis_rs::Iter<redis_rs::Value> = redis_rs::cmd("zrevrange")
            .arg(&key)
            .arg(0)
            .arg(first)
            .clone()
            .iter(&mut con)?;

        let mut changes = vec![];
        for value in iter {
            match value {
                redis_rs::Value::Data(data) => match serde_yaml::from_slice(&data) {
                    Ok(change) => changes.push(change),
                    Err(err) => log::error!("problem fetching change from redis: {}", err),
                },
                redis_rs::Value::Nil => {}
                other => {
                    log::error!("unexpected Redis value: {:?}", other);
                }
            }
        }

        Ok(changes)
    }
}
