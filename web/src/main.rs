mod config;
mod controllers;
mod serializers;
mod utils;
mod prelude;

use actix_web::{middleware, App, HttpServer};

use config::initializers;
use config::routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  println!("* Running initializers..");

  match initializers::run() {
    Ok((port, db_connection_pool)) => {
      println!("* Spinning up the application server..");

      let server = HttpServer::new(move || {
        App::new()
          .data(db_connection_pool.clone())
          .configure(routes::config)
          .wrap(middleware::Logger::default())
      }).bind(format!("0.0.0.0:{}", port))?;

      println!("* The application is running at 0.0.0.0:{}", port);

      server.run().await
    },
    Err(error) => {
      println!("{}", error);
      return Ok(());
    }
  }
}
