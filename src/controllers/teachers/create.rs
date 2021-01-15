use actix_web::{web, post, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use log::error;

use crate::{DbPool, services::teachers::{create, CreateError}};
use actix_web::rt::blocking::BlockingError;

#[derive(Deserialize)]
pub struct Params {
  email: String,
  password: String,
}

#[derive(Serialize)]
struct SuccessResponse {
}

#[derive(Serialize)]
struct ErrorResponse {
  errors: Vec<CreateError>,
}

#[post("")]
pub async fn action(db: web::Data<DbPool>, params: web::Json<Params>) -> impl Responder {
  let conn = db.get().expect("Couldn't get a database connection");
  let params = params.into_inner();

  match web::block(move || {
    create(params.email, params.password,&conn)
  }).await {
    Ok(_) => HttpResponse::Created().json(SuccessResponse { }),
    Err(BlockingError::Error(Some(errors))) => (
      HttpResponse::BadRequest().json(ErrorResponse { errors })
    ),
    Err(err) => {
      error!("{:?}", err);
      HttpResponse::InternalServerError().body("Unexpected error has occurred")
    },
  }
}