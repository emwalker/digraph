use async_graphql::*;

const WIKI_ID: &str = "45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb";

#[derive(Clone)]
pub enum Organization {
    #[allow(dead_code)]
    Wiki,
    Selected {
        id: ID,
        name: String,
        login: String,
    },
}

#[Object]
impl Organization {
    async fn id(&self) -> ID {
        match self {
            Self::Wiki => ID(WIKI_ID.to_string()),
            Self::Selected { id, .. } => id.to_owned(),
        }
    }

    async fn login(&self) -> &str {
        match self {
            Self::Wiki => "wiki",
            Self::Selected { login, .. } => login,
        }
    }

    async fn name(&self) -> &str {
        match self {
            Self::Wiki => "General",
            Self::Selected { name, .. } => name,
        }
    }
}
