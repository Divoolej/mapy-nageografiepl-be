use crate::prelude::*;

pub async fn handler() -> impl Responder {
  HttpResponse::Ok().body("Web server is alive.")
}
