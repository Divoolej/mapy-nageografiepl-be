use db::prelude::{create_database_connection_pool, DbPool};

pub fn init() -> Result<DbPool, String> {
  let pool = create_database_connection_pool()?;

  Ok(pool)
}
