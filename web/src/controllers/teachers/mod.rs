mod sessions;
mod create;

use crate::prelude::*;

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/teachers")
      .configure(sessions::config)
      .route("", web::post().to(create::handler))
  );
}
