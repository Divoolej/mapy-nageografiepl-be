use actix_web::{web, Scope};

mod create;
mod destroy;
mod refresh;

pub fn root() -> Scope {
  web::scope("/sessions")
    .service(create::action)
    .service(refresh::action)
    .service(destroy::action)
}
