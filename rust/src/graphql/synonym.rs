use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use serde_json::value::Value;

use crate::types::TimerangePrefix;

#[derive(Deserialize, Serialize, sqlx::Type, Clone, Debug, PartialEq, Eq, Hash)]
#[sqlx(transparent)]
pub struct Synonyms(pub Vec<Synonym>);

impl Synonyms {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn first(&self) -> Option<&Synonym> {
        self.0.first()
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Hash, SimpleObject)]
pub struct Synonym {
    #[serde(alias = "Name")]
    pub name: String,
    // TOOD: Change into a string enum
    #[serde(alias = "Locale")]
    pub locale: String,
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
    pub fn from_ref(input: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(input).map(Self)
    }

    pub fn display_name(&self, locale: &str, default: &str, prefix: &TimerangePrefix) -> String {
        let name = self
            .into_iter()
            .find(|s| s.locale == locale)
            .map(|s| s.name)
            .unwrap_or_else(|| default.to_owned());
        prefix.format(&name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, Utc};

    fn valid_date() -> Option<DateTime<Utc>> {
        DateTime::parse_from_rfc2822("Sat, 1 Jan 2000 00:00:00 +0000")
            .ok()
            .map(|dt| dt.with_timezone(&Utc))
    }

    #[test]
    fn from_ref() {
        let syn = Synonyms::from_ref(r#"[{"Name":"a","Locale":"en"}, {"Name":"b","Locale":"en"}]"#)
            .unwrap();
        assert_eq!(2, syn.into_iter().len());
    }

    #[test]
    fn simple_display_name() {
        let syn = Synonyms::from_ref(r#"[{"Name":"a","Locale":"en"}, {"Name":"b","Locale":"en"}]"#)
            .unwrap();
        assert_eq!(
            syn.display_name("en", "c", &TimerangePrefix::new(None, None)),
            "a"
        );
    }

    #[test]
    fn display_name_with_start_year_month_format() {
        let syn = Synonyms::from_ref(r#"[{"Name":"a","Locale":"en"}, {"Name":"b","Locale":"en"}]"#)
            .unwrap();
        assert_eq!(
            syn.display_name(
                "en",
                "c",
                &TimerangePrefix::new(Some("START_YEAR_MONTH"), valid_date())
            ),
            "2000-01 a"
        );
    }

    #[test]
    fn display_name_with_start_year_format() {
        let syn = Synonyms::from_ref(r#"[{"Name":"a","Locale":"en"}, {"Name":"b","Locale":"en"}]"#)
            .unwrap();
        assert_eq!(
            syn.display_name(
                "en",
                "c",
                &TimerangePrefix::new(Some("START_YEAR"), valid_date())
            ),
            "2000 a"
        );
    }
}
