use actix_web::{delete, rt::blocking::BlockingError, web, HttpResponse, Responder};

use crate::prelude::*;
use crate::services::teachers::sessions::{destroy, DestroyError, DestroyErrors};

#[derive(Deserialize)]
pub struct Params {
  refresh_token: String,
}

#[derive(Serialize)]
struct SuccessResponse {}

#[derive(Serialize)]
struct ErrorResponse {
  errors: Vec<DestroyError>,
}

#[delete("/{session_uuid}")]
pub async fn action(
  web::Path(session_uuid): web::Path<String>,
  db: web::Data<DbPool>,
  params: web::Json<Params>,
) -> impl Responder {
  let conn = db_connect!(db);
  let params = params.into_inner();

  match web::block(move || destroy(session_uuid, params.refresh_token, &conn)).await {
    Ok(_) => HttpResponse::Ok().json(SuccessResponse {}),
    Err(BlockingError::Error(service_errors)) => match service_errors {
      DestroyErrors::Multiple(errors) => HttpResponse::BadRequest().json(ErrorResponse { errors }),
      DestroyErrors::SessionNotFound => HttpResponse::NotFound().body("Not Found"),
      DestroyErrors::Unauthorized => HttpResponse::Unauthorized().body("Unauthorized"),
      DestroyErrors::UnexpectedError => http_500!(),
    },
    Err(BlockingError::Canceled) => http_500!(),
  }
}
