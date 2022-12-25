pub use super::errors::Error;
pub use super::http::RepoUrl;
pub use super::types::{
    Alert, Locale, ExternalId, Okey, RepoId, RepoIds, Result, Timerange, TimerangePrefixFormat, Timestamp,
    Viewer,
};

pub const API_VERSION: &str = "objects/v1";
pub const DEFAULT_ORGANIZATION_NAME: &str = "Personal org";
pub const DEFAULT_PRIVATE_COLOR: &str = "#5caeff";
pub const DEFAULT_REPOSITORY_NAME: &str = "Personal repo";
pub const DEFAULT_ROOT_TOPIC_NAME: &str = "Everything";
pub const GUEST_ID: &str = "11a13e26-ee64-4c31-8af1-d1e953899ee0";
pub const OTHER_REPOSITORY_ID: &str = "9f43f78c-9193-4ebf-b75c-edf362714baa";
pub const ROOT_TOPIC_ID: &str = "lBwR6Cvz4btdI23oscsp7THRytHohlol4o2IkqxcFN8";
pub const WIKI_ORGANIZATION_ID: &str = "45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb";
pub const WIKI_REPOSITORY_ID: &str = "32212616-fc1b-11e8-8eda-b70af6d8d09f";
