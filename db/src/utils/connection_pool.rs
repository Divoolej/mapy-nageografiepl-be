use std::time::Duration;
use diesel::r2d2::{Pool, ConnectionManager};
use crate::utils::types::{DbPool, DbConnection};

const DATABASE_CONNECTION_TIMEOUT: u64 = 2048;

pub fn create_database_connection_pool() -> Result<DbPool, String> {
  let database_url = std::env::var("DATABASE_URL")
    .map_err(|_| String::from("DATABASE_URL is not set!"))?;
  let connection_manager = ConnectionManager::<DbConnection>::new(database_url);
  let connection_pool = Pool::builder()
    .connection_timeout(Duration::from_millis(DATABASE_CONNECTION_TIMEOUT))
    .build(connection_manager)
    .map_err(|err| format!("Failed to create a database connection pool: {}", err))?;

  Ok(connection_pool)
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::utils::test::with_db;

  #[test]
  fn creating_database_connection_pool_works() {
    with_db(|_| {
      assert!(create_database_connection_pool().is_ok());
    });
  }
}
