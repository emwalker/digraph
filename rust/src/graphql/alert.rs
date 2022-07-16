use async_graphql::*;

use crate::types;

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

impl From<&types::Alert> for Alert {
    fn from(alert: &types::Alert) -> Self {
        let (alert_type, text) = match alert {
            types::Alert::Danger(text) => (AlertType::Error, text),
            types::Alert::Success(text) => (AlertType::Success, text),
            types::Alert::Warning(text) => (AlertType::Warn, text),
        };

        Alert {
            alert_type,
            id: String::from("0"),
            text: text.to_owned(),
        }
    }
}
