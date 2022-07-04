use sqlx::{Postgres, Transaction};

mod activity;
pub use activity::*;
mod link;
mod queries;
pub use link::*;
mod organization;
pub use organization::*;
mod registration;
pub use registration::*;
mod repository;
pub use repository::*;
mod session;
pub use session::*;
pub mod user;
pub use user::*;

type PgTransaction<'t> = Transaction<'t, Postgres>;
