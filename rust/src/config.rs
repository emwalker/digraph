use std::path::Path;
use serde::Deserialize;

use super::prelude::*;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub digraph_postgres_connection: String,
    pub digraph_server_secret: String,
    pub rust_log: String,
    pub session_domain: String,
    pub session_key: String,
}

impl Config {
    pub fn load() -> Result<Self> {
        let profile = if cfg!(debug_assertions) {
            "development"
        } else {
            "production"
        };

        let filename = format!(".env.{}.local", profile);
        if Path::new(&filename).exists() {
            dotenv::from_filename(filename).ok();
        }
        dotenv::dotenv().ok();

        envy::from_env::<Self>().map_err(Error::from)
    }
}
