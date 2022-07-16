use async_graphql::connection::*;
use async_graphql::{connection::EmptyFields, scalar, Enum, SimpleObject};
use serde::{Deserialize, Serialize};

use crate::types;

#[derive(Enum, Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum TimerangePrefixFormat {
    None,
    StartYear,
    StartYearMonth,
}

impl std::fmt::Display for TimerangePrefixFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let string = match self {
            Self::None => "NONE",
            Self::StartYear => "START_YEAR",
            Self::StartYearMonth => "START_YEAR_MONTH",
        };
        write!(f, "{}", string)
    }
}

impl From<&types::TimerangePrefixFormat> for TimerangePrefixFormat {
    fn from(format: &types::TimerangePrefixFormat) -> Self {
        match format {
            types::TimerangePrefixFormat::None => Self::None,
            types::TimerangePrefixFormat::StartYear => Self::StartYear,
            types::TimerangePrefixFormat::StartYearMonth => Self::StartYearMonth,
        }
    }
}

impl From<&TimerangePrefixFormat> for types::TimerangePrefixFormat {
    fn from(prefix_format: &TimerangePrefixFormat) -> Self {
        match prefix_format {
            TimerangePrefixFormat::None => types::TimerangePrefixFormat::None,
            TimerangePrefixFormat::StartYear => types::TimerangePrefixFormat::StartYear,
            TimerangePrefixFormat::StartYearMonth => types::TimerangePrefixFormat::StartYearMonth,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct DateTime(pub chrono::DateTime<chrono::Utc>);
scalar!(DateTime);

#[derive(Clone, Debug, Eq, Hash, PartialEq, SimpleObject)]
pub struct Timerange {
    pub ends_at: Option<DateTime>,
    pub prefix_format: TimerangePrefixFormat,
    pub starts_at: DateTime,
}

impl From<&types::Timerange> for Timerange {
    fn from(timerange: &types::Timerange) -> Self {
        Self {
            ends_at: None,
            starts_at: DateTime(timerange.starts),
            prefix_format: TimerangePrefixFormat::from(&timerange.prefix_format),
        }
    }
}

pub type TimerangeEdge = Edge<String, Timerange, EmptyFields>;
