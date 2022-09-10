use redis_rs::{self, Commands};
use std::collections::{BTreeSet, HashMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::git;
use crate::prelude::*;
use crate::types::{Downset, ReadPath};

#[derive(Clone, Debug)]
pub struct Noop;

impl git::SaveChangesForPrefix for Noop {
    fn save(
        &self,
        _prefix: &RepoId,
        _changes: &HashMap<RepoId, BTreeSet<git::activity::Change>>,
    ) -> Result<()> {
        // Do nothing
        Ok(())
    }
}

impl git::CacheStats for Noop {
    fn fetch(&self, _repo: &RepoId, _commit: &str) -> Result<Option<git::RepoStats>> {
        Ok(None)
    }

    fn save(
        &self,
        _repo: &RepoId,
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
    fn downset(path: &ReadPath) -> Self {
        Self(format!("topic:{}:{}:down", path.id, path.commit))
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

impl git::CacheStats for Redis {
    fn fetch(&self, repo: &RepoId, commit: &str) -> Result<Option<git::RepoStats>> {
        let key = self.stats_key(repo, commit);
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
        repo: &RepoId,
        commit: &str,
        stats: &git::RepoStats,
        ttl: Option<u32>,
    ) -> Result<()> {
        let key = self.stats_key(repo, commit);
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

    pub fn intersection<F>(&self, fetch: &F, paths: &[ReadPath]) -> Result<HashSet<Oid>>
    where
        F: Downset,
    {
        if paths.is_empty() {
            log::warn!("no paths provided for transitive closure, exiting early");
            return Ok(HashSet::new());
        }

        let (head, tail) = paths.split_at(1);
        let mut con = self.connection()?;
        let mut keys = vec![];

        match head.get(0) {
            Some(path) => {
                let key = Key::downset(path);
                keys.push(key.clone());

                if !con.exists(&key)? {
                    log::info!("redis: {:?} not found in redis, saving", key);
                    self.save_downset(&mut con, &key, &fetch.downset(path))?;
                }

                for other_path in tail {
                    let key = Key::downset(other_path);
                    keys.push(key.clone());

                    if !con.exists(&key)? {
                        log::info!("redis: {:?} not found in redis, saving", key);
                        self.save_downset(&mut con, &key, &fetch.downset(other_path))?;
                    }
                }

                let set: HashSet<String> = con.sinter(&keys)?;
                Ok(set
                    .iter()
                    .map(Oid::try_from)
                    .collect::<Result<HashSet<Oid>>>()?)
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
        set: &HashSet<Oid>,
    ) -> Result<()> {
        redis_rs::transaction(con, &[key], |con, pipe| {
            let set = set.iter().map(Oid::to_string).collect::<HashSet<String>>();
            if set.is_empty() {
                pipe.ignore()
            } else {
                pipe.sadd(key, set).ignore()
            }
            .query(con)
        })?;

        Ok(())
    }

    fn stats_key(&self, repo: &RepoId, commit: &str) -> String {
        format!("stats:{}:{}", repo, commit)
    }
}

impl git::SaveChangesForPrefix for Redis {
    fn save(
        &self,
        prefix: &RepoId,
        prefix_changes: &HashMap<RepoId, BTreeSet<git::activity::Change>>,
    ) -> Result<()> {
        let mut con = self.connection()?;
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let empty = BTreeSet::new();
        let changes = prefix_changes.get(prefix).unwrap_or(&empty);

        let mut args = vec![];
        for change in changes {
            let string = serde_yaml::to_string(change)?;
            args.push((now, string));
        }

        let key = Key(format!("activity:{}", prefix));
        if !args.is_empty() {
            log::info!("saving changes to {:?}", key);
            con.zadd_multiple(key, &args)?;
        } else {
            log::info!("no changes to save to {:?} key, skipping", key);
        }

        Ok(())
    }
}

impl git::activity::ActivityForPrefix for Redis {
    fn fetch_activity(&self, prefix: &RepoId, first: usize) -> Result<Vec<git::activity::Change>> {
        let key = Key(format!("activity:{}", prefix));
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
