use async_graphql::{Object, ID};

use crate::prelude::*;

#[derive(Clone)]
pub enum Organization {
    #[allow(dead_code)]
    Wiki,
    Selected {
        id: ID,
        login: String,
        name: String,
    },
}

impl Default for Organization {
    fn default() -> Self {
        Self::Wiki
    }
}

#[Object]
impl Organization {
    async fn login(&self) -> &str {
        match self {
            Self::Wiki => "wiki",
            Self::Selected { login, .. } => login,
        }
    }

    async fn id(&self) -> ID {
        match self {
            Self::Wiki => ID(WIKI_ORGANIZATION_ID.to_owned()),
            Self::Selected { id, .. } => id.to_owned(),
        }
    }

    async fn name(&self) -> &str {
        match self {
            Self::Wiki => "General",
            Self::Selected { name, .. } => name,
        }
    }

    async fn path(&self) -> String {
        match self {
            Self::Wiki => "wiki".to_string(),
            Self::Selected { login, .. } => login.to_owned(),
        }
    }
}
