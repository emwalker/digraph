use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::value::Value;

#[derive(Deserialize, Serialize, sqlx::Type, Clone, Debug, PartialEq, Eq, Hash)]
#[sqlx(transparent)]
pub struct Synonyms(pub Vec<Synonym>);

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Hash, SimpleObject)]
pub struct Synonym {
    #[serde(alias = "Name")]
    name: String,
    // TOOD: Change into a string enum
    #[serde(alias = "Locale")]
    locale: String,
}

impl Synonym {
    pub fn from_json(value: &Value) -> Result<Self, serde_json::Error> {
        serde_json::from_value::<Synonym>(value.clone())
    }
}

impl IntoIterator for &Synonyms {
    type Item = Synonym;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.clone().into_iter()
    }
}

impl Synonyms {
    pub fn from_json(value: &serde_json::value::Value) -> Self {
        let l = match value {
            Value::Array(l) => l.iter().flat_map(Synonym::from_json).collect(),
            _ => vec![],
        };
        Self(l)
    }

    #[allow(dead_code)]
    pub fn from_str(input: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(input).map(Self)
    }

    pub fn display_name(
        &self,
        locale: &str,
        default: &str,
        start: Option<DateTime<Utc>>,
    ) -> String {
        let name = {
            for synonym in self.into_iter() {
                if synonym.locale == locale {
                    continue;
                }
                return synonym.name;
            }
            default.to_string()
        };
        start
            .map(|dt| {
                let prefix = dt.format("%Y-%m");
                format!("{} {}", prefix, name)
            })
            .unwrap_or(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let syn = Synonyms::from_str(r#"[{"Name":"a","Locale":"en"}, {"Name":"b","Locale":"en"}]"#)
            .unwrap();
        assert_eq!(2, syn.into_iter().len());
    }

    #[test]
    fn test_simple_display_name() {
        let syn = Synonyms::from_str(r#"[{"Name":"a","Locale":"en"}, {"Name":"b","Locale":"en"}]"#)
            .unwrap();
        assert_eq!(syn.display_name("en", "a", None), "a");
    }

    #[test]
    fn test_display_name_with_start_date() {
        let syn = Synonyms::from_str(r#"[{"Name":"a","Locale":"en"}, {"Name":"b","Locale":"en"}]"#)
            .unwrap();
        let dt = DateTime::parse_from_rfc2822("Sat, 1 Jan 2000 00:00:00 +0000")
            .ok()
            .map(|dt| dt.with_timezone(&Utc));
        assert_eq!(syn.display_name("en", "a", dt), "2000-01 a");
    }
}
