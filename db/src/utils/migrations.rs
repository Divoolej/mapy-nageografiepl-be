use diesel::Connection;
use crate::utils::types::DbConnection;

embed_migrations!();

pub fn run_migrations() -> Result<(), String> {
  let database_url = std::env::var("DATABASE_URL")
    .map_err(|_| String::from("DATABASE_URL is not set!"))?;
  let db_conn = DbConnection::establish(&database_url)
    .map_err(|err| format!("Failed to establish database connection: {}", err))?;
  embedded_migrations::run_with_output(&db_conn, &mut std::io::stdout())
    .map_err(|err| format!("Migrations failed with an error: {}", err))?;
  Ok(())
}
