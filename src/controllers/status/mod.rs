mod web_healthcheck;

use actix_web::{web, Scope};

pub fn root() -> Scope {
  web::scope("/status").service(web_healthcheck::action)
}
