use actix_identity::Identity;
use serde::Deserialize;

use super::GUEST_ID;
use crate::prelude::*;

#[derive(Clone, Deserialize, SimpleObject)]
pub struct Session {
    pub id: String,
}

impl Session {
    pub fn from(id: &Identity) -> Result<Self> {
        let session = match id.identity() {
            Some(string) => serde_json::from_str::<Self>(&string)?,
            None => Self {
                id: GUEST_ID.to_string(),
            },
        };
        Ok(session)
    }
}
