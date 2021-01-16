#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rollbar;
#[macro_use]
extern crate lazy_static;

mod controllers;
mod models;
mod schema;
mod services;
mod utils;

use actix_web::{middleware, web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use std::env;
use std::time::Duration;

use crate::utils::errors::ROLLBAR_CLIENT;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub mod prelude {
  pub use crate::utils::errors::ROLLBAR_CLIENT;
  pub use crate::DbPool;
  pub use crate::{db_connect, handle_unexpected_err, http_500, make_serializable, report_unexpected_err};
  pub use log::error;
  pub use serde::{Deserialize, Serialize, Serializer};
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  // Load ENVs
  env::set_var("RUST_LOG", "debug");
  dotenv::dotenv().ok();
  // Set up logger
  env_logger::init();
  // Set up Rollbar
  report_panics!(ROLLBAR_CLIENT);
  // Set up PostgreSQL connection pool
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
  let connection_manager = ConnectionManager::<PgConnection>::new(database_url);
  let connection_pool = Pool::builder()
    .connection_timeout(Duration::from_millis(2048))
    .build(connection_manager)
    .expect("Failed to create a database connection pool");

  HttpServer::new(move || {
    App::new()
      .data(connection_pool.clone())
      .wrap(middleware::Logger::default())
      .service(
        web::scope("/api/v1")
          .service(controllers::status::root())
          .service(controllers::teachers::root()),
      )
  })
  .bind("0.0.0.0:3000")?
  .run()
  .await
}
