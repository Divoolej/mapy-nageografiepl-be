mod web_healthcheck;

use crate::prelude::*;

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/status")
      .service(
        web::resource("/web")
          .route(web::get().to(web_healthcheck::handler))
      )
  );
}
