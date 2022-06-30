use async_graphql::*;

#[derive(Clone, Copy, Debug, Enum, PartialEq, Eq)]
pub enum AlertType {
    Success,
    Warn,
    Error,
}

impl std::fmt::Display for AlertType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Success => write!(f, "success"),
            Self::Warn => write!(f, "warn"),
            Self::Error => write!(f, "error"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Alert {
    pub text: String,
    pub alert_type: AlertType,
    pub id: String,
}

#[Object]
impl Alert {
    #[graphql(name = "type")]
    async fn alert_type(&self) -> AlertType {
        self.alert_type
    }

    async fn id(&self) -> String {
        self.id.to_owned()
    }

    async fn text(&self) -> String {
        self.text.to_owned()
    }
}

pub fn success(text: String) -> Alert {
    Alert {
        text,
        alert_type: AlertType::Success,
        id: String::from("0"),
    }
}

pub fn warning(text: String) -> Alert {
    Alert {
        text,
        alert_type: AlertType::Warn,
        id: String::from("0"),
    }
}