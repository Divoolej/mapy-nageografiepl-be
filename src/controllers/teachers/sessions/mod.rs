use actix_web::{web, Scope};

mod create;
mod refresh;

pub fn root() -> Scope {
  web::scope("/sessions").service(create::action).service(refresh::action)
}
