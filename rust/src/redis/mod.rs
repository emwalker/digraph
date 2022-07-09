use redis_rs::{self, Commands};
use std::collections::{HashMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::git;
use crate::prelude::*;
use crate::DownSet;

pub struct Noop;

impl git::SaveChangesForPrefix for Noop {
    fn save(
        &self,
        _prefix: &str,
        _changes: &HashMap<String, Vec<git::desc::Change>>,
    ) -> Result<()> {
        // Do nothing
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Key(pub String);

impl redis_rs::ToRedisArgs for Key {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis_rs::RedisWrite,
    {
        out.write_arg(self.0.as_bytes());
    }
}

fn down_set_key(path: &RepoPath) -> Key {
    Key(format!("topic:{}:down", path))
}

#[derive(Clone)]
pub struct Redis {
    url: String,
}

impl Redis {
    pub fn new(url: String) -> Result<Self> {
        Ok(Redis { url })
    }

    pub fn transitive_closure<F>(&self, fetch: &F, paths: &[&RepoPath]) -> Result<HashSet<String>>
    where
        F: DownSet,
    {
        let (head, tail) = paths.split_at(1);
        let mut con = self.connection()?;
        let mut keys = vec![];

        match head.get(0) {
            Some(path) => {
                let key = down_set_key(path);
                keys.push(key.clone());

                if !con.exists(&key)? {
                    self.save_set(&mut con, &key, &fetch.down_set(*path))?;
                }

                for other_path in tail {
                    let key = down_set_key(other_path);
                    keys.push(key.clone());

                    if !con.exists(&key)? {
                        self.save_set(&mut con, &key, &fetch.down_set(*other_path))?;
                    }
                }

                Ok(con.sinter(&keys)?)
            }

            None => Ok(HashSet::new()),
        }
    }

    pub fn connection(&self) -> Result<redis_rs::Connection> {
        let client = redis_rs::Client::open(self.url.clone())?;
        Ok(client.get_connection()?)
    }

    // Within a transaction, write a set to a key and configure the key to expire in 60 seconds.
    fn save_set(
        &self,
        con: &mut redis_rs::Connection,
        key: &Key,
        set: &HashSet<String>,
    ) -> Result<()> {
        let _ = redis_rs::transaction(con, &[key], |con, pipe| {
            pipe.sadd(key, set).ignore().expire(key, 60).query(con)
        })?;

        Ok(())
    }
}

impl git::SaveChangesForPrefix for Redis {
    fn save(
        &self,
        prefix: &str,
        prefix_changes: &HashMap<String, Vec<git::desc::Change>>,
    ) -> Result<()> {
        let mut con = self.connection()?;
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let empty = vec![];
        let changes = prefix_changes.get(prefix).unwrap_or(&empty);

        let mut args = vec![];
        for change in changes {
            let string = serde_yaml::to_string(change)?;
            args.push((now, string));
        }

        let key = Key(format!("activity:{}", prefix));
        log::info!("saving changes to {:?}", key);
        let _ = con.zadd_multiple(key, &args)?;
        Ok(())
    }
}

impl git::ActivityForPrefix for Redis {
    fn fetch_activity(&self, prefix: &str) -> Result<Vec<git::desc::Change>> {
        let key = Key(format!("activity:{}", prefix));
        log::info!("fetching activity for prefix {:?} from Redis", key);
        let mut con = self.connection()?;

        let iter: redis_rs::Iter<redis_rs::Value> = redis_rs::cmd("zrevrange")
            .arg(&key)
            .arg(0)
            .arg(3)
            .clone()
            .iter(&mut con)?;

        let mut changes = vec![];
        for value in iter {
            match value {
                redis_rs::Value::Data(data) => {
                    let change = serde_yaml::from_slice(&data)?;
                    changes.push(change);
                }
                redis_rs::Value::Nil => {}
                other => {
                    log::error!("unexpected Redis value: {:?}", other);
                }
            }
        }

        Ok(changes)
    }
}
