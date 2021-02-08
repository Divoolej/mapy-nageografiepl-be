use crate::prelude::*;

mod create;
mod destroy;
mod refresh;

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/sessions")
      .route("", web::post().to(create::handler))
      .service(
        web::scope("/{session_uuid}")
          .route("", web::delete().to(destroy::handler))
          .route("/refresh", web::patch().to(refresh::handler))
      )
  );
}
