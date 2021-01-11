use actix_web::{
  get,
  Responder,
  HttpResponse
};

#[get("/web")]
pub async fn action() -> impl Responder {
  HttpResponse::Ok().body("Web server is alive.")
}

