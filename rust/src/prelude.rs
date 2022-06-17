pub use super::errors::Error;
pub use async_graphql::{Context, Object, SimpleObject, ID};

pub type Result<T> = std::result::Result<T, Error>;
