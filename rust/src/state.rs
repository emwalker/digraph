use sqlx::postgres::PgPool;
use std::sync::Arc;

use crate::psql;
use crate::server::services::{link, topic};

pub struct State {
    pub links: link::Service,
    pub topics: topic::Service,
}

impl State {
    pub fn new(pool: PgPool) -> Self {
        let topics = topic::Service {
            repo: Arc::new(psql::topic::Repo::new(pool.clone())),
        };
        let links = link::Service {
            repo: Arc::new(psql::link::Repo::new(pool)),
        };

        Self { links, topics }
    }
}
