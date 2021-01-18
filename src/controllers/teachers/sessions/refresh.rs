use actix_web::{patch, rt::blocking::BlockingError, web, HttpRequest, HttpResponse, Responder};
use chrono::{DateTime, Utc};

use crate::prelude::*;
use crate::services::teachers::sessions::{refresh, RefreshError, RefreshErrors};

#[derive(Serialize)]
struct SuccessResponse {
  access_token: String,
  access_token_expires_at: DateTime<Utc>,
}

#[derive(Serialize)]
struct ErrorResponse {
  errors: Vec<RefreshError>,
}

#[patch("/{session_uuid}/refresh")]
pub async fn action(
  request: HttpRequest,
  web::Path(session_uuid): web::Path<String>,
  db: web::Data<DbPool>,
) -> impl Responder {
  let conn = db_connect!(db);
  let refresh_token: String = require_refresh_token!(request);

  match web::block(move || refresh(session_uuid, refresh_token, &conn)).await {
    Ok(session) => HttpResponse::Ok().json(SuccessResponse {
      access_token: session.access_token,
      access_token_expires_at: session.access_token_expires_at,
    }),
    Err(BlockingError::Error(service_errors)) => match service_errors {
      RefreshErrors::Multiple(errors) => HttpResponse::BadRequest().json(ErrorResponse { errors }),
      RefreshErrors::SessionNotFound => HttpResponse::NotFound().body("Not Found"),
      RefreshErrors::Unauthorized => HttpResponse::Unauthorized().body("Unauthorized"),
      RefreshErrors::UnexpectedError => http_500!(),
    },
    Err(BlockingError::Canceled) => http_500!(),
  }
}
