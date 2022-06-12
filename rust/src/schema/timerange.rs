use async_graphql::{scalar, Enum, SimpleObject};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Prefix {
    None,
    StartYear(chrono::DateTime<chrono::Utc>),
    StartYearMonth(chrono::DateTime<chrono::Utc>),
}

impl Prefix {
    pub fn new(
        prefix_format: Option<&str>,
        starts_at: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Self {
        match prefix_format {
            Some(format) => match starts_at {
                Some(starts_at) => match format {
                    "START_YEAR" => Self::StartYear(starts_at),
                    "START_YEAR_MONTH" => Self::StartYearMonth(starts_at),
                    _ => Self::None,
                },
                None => Self::None,
            },
            None => Self::None,
        }
    }

    pub fn display(&self, name: &str) -> String {
        match self {
            Self::None => name.to_owned(),
            Self::StartYear(starts_at) => {
                let prefix = starts_at.format("%Y");
                format!("{} {}", prefix, name)
            }
            Self::StartYearMonth(starts_at) => {
                let prefix = starts_at.format("%Y-%m");
                format!("{} {}", prefix, name)
            }
        }
    }

    pub fn to_format(&self) -> TimeRangePrefixFormat {
        match self {
            Self::None => TimeRangePrefixFormat::None,
            Self::StartYear(_) => TimeRangePrefixFormat::StartYear,
            Self::StartYearMonth(_) => TimeRangePrefixFormat::StartYearMonth,
        }
    }
}

#[derive(Enum, Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum TimeRangePrefixFormat {
    None,
    StartYear,
    StartYearMonth,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct DateTime(pub chrono::DateTime<chrono::Utc>);
scalar!(DateTime);

#[derive(Clone, Debug, Eq, Hash, PartialEq, SimpleObject)]
pub struct TimeRange {
    pub starts_at: DateTime,
    pub ends_at: Option<DateTime>,
    pub prefix_format: TimeRangePrefixFormat,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_date() -> Option<chrono::DateTime<chrono::Utc>> {
        chrono::DateTime::parse_from_rfc2822("Sat, 1 Jan 2000 00:00:00 +0000")
            .ok()
            .map(|dt| dt.with_timezone(&chrono::Utc))
    }

    #[test]
    fn test_none() {
        let prefix = Prefix::new(None, None);
        assert_eq!(prefix.display("a"), "a");
    }

    #[test]
    fn test_prefix_none() {
        let prefix = Prefix::new(Some("NONE"), valid_date());
        assert_eq!(prefix.display("a"), "a");
    }

    #[test]
    fn test_start_year() {
        let prefix = Prefix::new(Some("START_YEAR"), valid_date());
        assert_eq!(prefix.display("a"), "2000 a");
    }

    #[test]
    fn test_start_year_month() {
        let prefix = Prefix::new(Some("START_YEAR_MONTH"), valid_date());
        assert_eq!(prefix.display("a"), "2000-01 a");
    }
}
