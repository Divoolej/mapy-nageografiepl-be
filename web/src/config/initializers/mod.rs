mod db_connection_pool;
mod environment;
mod logger;
mod migrations;
mod port;
mod rollbar;

use crate::prelude::DbPool;

pub fn run() -> Result<(String, DbPool), String> {
  environment::init();
  logger::init()?;
  rollbar::init();
  migrations::init()?;
  let db_pool = db_connection_pool::init()?;
  let port = port::init();

  Ok((port, db_pool))
}
