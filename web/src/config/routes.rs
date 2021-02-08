use actix_web::web;

use crate::controllers::status;
use crate::controllers::teachers;

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/api")
      .service(
        web::scope("/v1")
          .configure(status::config)
          .configure(teachers::config)
      )
  );
}
