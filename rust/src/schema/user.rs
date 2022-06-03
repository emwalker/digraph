use async_graphql::*;

static GUEST_ID: &str = "11a13e26-ee64-4c31-8af1-d1e953899ee0";

#[derive(Clone)]
pub enum User {
    Guest,
    Registered {
        avatar_url: String,
        id: ID,
        name: String,
    },
}

#[Object]
impl User {
    pub async fn avatar_url(&self) -> Option<String> {
        match self {
            Self::Guest => None,
            Self::Registered { avatar_url, .. } => Some(avatar_url.to_owned()),
        }
    }

    pub async fn id(&self) -> ID {
        match self {
            Self::Guest => ID(GUEST_ID.to_string()),
            Self::Registered { id, .. } => id.to_owned(),
        }
    }

    pub async fn is_guest(&self) -> bool {
        match self {
            Self::Guest => true,
            Self::Registered { .. } => false,
        }
    }

    pub async fn name(&self) -> String {
        match self {
            Self::Guest => "Guest".to_owned(),
            Self::Registered { name, .. } => name.to_owned(),
        }
    }
}
