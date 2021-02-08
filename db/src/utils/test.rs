use std::panic::UnwindSafe;

use diesel::{Connection, RunQueryDsl};

use crate::utils::types::DbConnection;
use crate::repositories::Repository;
use crate::schema;

lazy_static! {
  pub static ref GLOBAL_SETUP: Option<bool> = {
    dotenv::dotenv().ok();
    std::env::set_var("DATABASE_URL", std::env::var("TEST_DATABASE_URL").unwrap());
    crate::utils::migrations::run_migrations().unwrap();
    Some(true)
  };
}

fn test_database_connection() -> DbConnection {
  let database_url = std::env::var("TEST_DATABASE_URL").unwrap();
  DbConnection::establish(&database_url).unwrap()
}

pub fn with_db<F>(f: F)
where
  F: FnOnce(DbConnection) + UnwindSafe
{
  GLOBAL_SETUP.unwrap();
  let result = std::panic::catch_unwind(|| f(test_database_connection()));
  db_cleanup();
  assert!(result.is_ok());
}

// pub fn with_repository<'a, R, F>(f: F)
// where
//   R: Repository<'a>,
//   F: FnOnce(R) + UnwindSafe,
// {
//   with_db(|connection| {
//     let repository = R::new(&connection);
//     f(repository);
//   })
// }

pub fn with_repository<R, F>(f: F)
where
  R: for<'a> Repository<'a>,
  F: FnOnce(R, &DbConnection),
{
  let connection = test_database_connection();
  let repository = R::new(&connection);
  f(repository, &connection);
}

fn db_cleanup() {
  let connection = test_database_connection();

  diesel::delete(schema::sessions::table)
    .execute(&connection)
    .expect("Failed to clean up sessions!");

  diesel::delete(schema::teachers::table)
    .execute(&connection)
    .expect("Failed to clean up teachers!");
}
