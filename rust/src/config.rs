use serde::Deserialize;
use std::path::Path;

use super::prelude::*;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub digraph_data_directory: String,
    pub digraph_postgres_connection: String,
    pub digraph_redis_url: String,
    pub digraph_server_secret: String,
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
