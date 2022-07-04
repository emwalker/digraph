extern crate strum;
extern crate strum_macros;

#[macro_use]
extern crate quick_error;

#[macro_use]
extern crate derivative;

use serde::{Deserialize, Serialize};

use strum_macros::EnumString;

pub mod config;
pub mod db;
pub mod errors;
pub mod git;
pub mod graphql;
pub mod http;
pub mod prelude;
mod psql;
pub mod repo;

pub enum Alert {
    Danger(String),
    Success(String),
    Warning(String),
}

#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    EnumString,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    strum_macros::Display,
)]
#[serde(rename_all = "lowercase")]
pub enum Locale {
    EN,
    AR,
    DE,
    EL,
    ES,
    FA,
    FI,
    FR,
    HI,
    IT,
    JA,
    JI,
    KO,
    LA,
    NL,
    NO,
    PT,
    RU,
    SV,
    TR,
    UA,
    UK,
    ZH,
}
