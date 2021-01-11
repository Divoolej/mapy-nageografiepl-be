#[macro_use] extern crate diesel;

use actix_web::{web, middleware, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{Pool, ConnectionManager};

mod utils;
mod schema;
mod models;
mod services;
mod controllers;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  // Load ENVs
  dotenv::dotenv().ok();
  // Set up PostgreSQL connection pool
  let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  let connection_manager = ConnectionManager::<PgConnection>::new(database_url);
  let connection_pool = Pool::builder()
    .build(connection_manager)
    .expect("Failed to create a database connection pool");

  HttpServer::new(move || {
    App::new()
      .data(connection_pool.clone())
      .wrap(middleware::Logger::default())
      .service(web::scope("/api/v1")
          .service(controllers::status::root())
          .service(controllers::teachers::root())
      )
  })
  .bind("0.0.0.0:3000")?
  .run()
  .await
}
