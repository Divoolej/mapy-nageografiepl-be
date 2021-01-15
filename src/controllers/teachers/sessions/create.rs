use actix_web::{web, post, HttpResponse, rt::blocking::BlockingError, Responder};
use serde::{Serialize, Deserialize};

use crate::{
  db_connect,
  DbPool,
  services::teachers::sessions::{
    create,
    CreateError,
  },
};

#[derive(Deserialize)]
pub struct Params {
  email: String,
  password: String,
}

#[derive(Serialize)]
struct ErrorResponse {
  errors: Vec<CreateError>,
}

#[post("")]
pub async fn action(db: web::Data<DbPool>, params: web::Json<Params>) -> impl Responder {
  let conn = db_connect!(db);
  let params = params.into_inner();

  match web::block(move || {
    create(params.email, params.password,&conn)
  }).await {
    Ok(session) => HttpResponse::Created().json(session),
    Err(BlockingError::Error(Some(errors))) => (
        HttpResponse::BadRequest().json(ErrorResponse { errors })
    ),
    Err(err) => (
      HttpResponse::InternalServerError().body("Unexpected error has occurred")
    ),
  }
}