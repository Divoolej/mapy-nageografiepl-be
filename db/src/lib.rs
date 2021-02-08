#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate lazy_static;

mod schema;
pub mod utils;
pub mod models;
pub mod repositories;

pub mod prelude {
  pub use crate::utils::types::{DbPool, DbConnection};
  pub use crate::utils::errors::DbError;
  pub use crate::utils::connection_pool::create_database_connection_pool;
  pub use crate::utils::migrations::run_migrations;
  pub use crate::repositories::{Repository, SessionsRepository, TeachersRepository};
}
