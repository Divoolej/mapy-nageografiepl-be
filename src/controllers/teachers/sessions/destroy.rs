use actix_web::{delete, rt::blocking::BlockingError, web, HttpRequest, HttpResponse, Responder};

use crate::prelude::*;
use crate::services::teachers::sessions::{destroy, DestroyError, DestroyErrors};

#[derive(Serialize)]
struct SuccessResponse {}

#[derive(Serialize)]
struct ErrorResponse {
  errors: Vec<DestroyError>,
}

#[delete("/{session_uuid}")]
pub async fn action(
  request: HttpRequest,
  web::Path(session_uuid): web::Path<String>,
  db: web::Data<DbPool>,
) -> impl Responder {
  let conn = db_connect!(db);
  let refresh_token: String = require_refresh_token!(request);

  match web::block(move || destroy(session_uuid, refresh_token, &conn)).await {
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
