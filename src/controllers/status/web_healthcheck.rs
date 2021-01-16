use actix_web::{get, HttpResponse, Responder};

#[get("/web")]
pub async fn action() -> impl Responder {
  HttpResponse::Ok().body("Web server is alive.")
}
