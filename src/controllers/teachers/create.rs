use actix_web::{web, post, rt::blocking::BlockingError, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use log::error;

use crate::{
  db_connect, http_500,
  DbPool,
  services::teachers::{
    create,
    CreateError,
    CreateErrors,
  },
};

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
  let conn = db_connect!(db);
  let params = params.into_inner();

  match web::block(move || {
    create(params.email, params.password,&conn)
  }).await {
    Ok(_) => HttpResponse::Created().json(SuccessResponse { }),
    Err(BlockingError::Error(service_errors)) => match service_errors {
      CreateErrors::Multiple(errors) => HttpResponse::BadRequest().json(ErrorResponse { errors }),
      CreateErrors::UnexpectedError => http_500!(),
    },
    Err(BlockingError::Canceled) => http_500!(),
  }
}