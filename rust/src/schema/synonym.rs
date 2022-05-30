use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use serde_json::Value::Array;

#[derive(Deserialize, Serialize, sqlx::Type, Clone, Debug)]
#[sqlx(transparent)]
pub struct Synonyms(pub serde_json::Value);

#[derive(Clone, Deserialize, Debug, SimpleObject)]
pub struct Synonym {
    #[serde(alias = "Name")]
    name: String,
    // TOOD: Change into a string enum
    #[serde(alias = "Locale")]
    locale: String,
}

impl Synonyms {
    #[allow(dead_code)]
    pub fn from_str(input: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(input).map(Self)
    }

    pub fn to_vec(&self) -> Vec<Synonym> {
        match &self.0 {
            Array(l) => l
                .iter()
                .flat_map(|v| serde_json::from_value::<Synonym>(v.clone()).ok())
                .collect::<Vec<Synonym>>(),
            _ => vec![],
        }
    }

    pub fn display_name(
        &self,
        locale: &str,
        default: &str,
        start: Option<DateTime<Utc>>,
    ) -> String {
        let name = {
            for synonym in self.to_vec() {
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
        assert_eq!(2, syn.to_vec().len());
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
