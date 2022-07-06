use redis_rs::{self, Commands};
use std::collections::HashSet;

use crate::prelude::*;
use crate::DownSet;

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

pub struct Redis<T: redis_rs::IntoConnectionInfo + Clone> {
    connection_info: T,
}

impl<T: redis_rs::IntoConnectionInfo + Clone> Redis<T> {
    pub fn new(connection_info: T) -> Self {
        Redis { connection_info }
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

    fn connection(&self) -> Result<redis_rs::Connection> {
        let client = redis_rs::Client::open(self.connection_info.clone())?;
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
