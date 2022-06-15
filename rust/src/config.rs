use serde::Deserialize;

use super::prelude::*;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub rust_log: String,
    pub digraph_server_secret: String,
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

        dotenv::from_filename(format!(".env.{}.local", profile)).ok();
        dotenv::dotenv().ok();

        envy::from_env::<Self>().map_err(Error::from)
    }
}
