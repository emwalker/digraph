use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Prefix {
    None,
    StartYear(DateTime<Utc>),
    StartYearMonth(DateTime<Utc>),
}

impl Prefix {
    pub fn new(prefix_format: Option<&str>, starts_at: Option<DateTime<Utc>>) -> Self {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_date() -> Option<DateTime<Utc>> {
        DateTime::parse_from_rfc2822("Sat, 1 Jan 2000 00:00:00 +0000")
            .ok()
            .map(|dt| dt.with_timezone(&Utc))
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
