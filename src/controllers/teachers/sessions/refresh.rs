use actix_web::{patch, rt::blocking::BlockingError, web, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use log::error;
use serde::{Deserialize, Serialize};

use crate::{
    db_connect, http_500,
    services::teachers::sessions::{refresh, RefreshError, RefreshErrors},
    DbPool,
};

#[derive(Deserialize)]
pub struct Params {
    refresh_token: String,
}

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
    web::Path(session_uuid): web::Path<String>,
    db: web::Data<DbPool>,
    params: web::Json<Params>,
) -> impl Responder {
    let conn = db_connect!(db);
    let params = params.into_inner();

    match web::block(move || refresh(session_uuid, params.refresh_token, &conn)).await {
        Ok(session) => HttpResponse::Ok().json(SuccessResponse {
            access_token: session.access_token,
            access_token_expires_at: session.access_token_expires_at,
        }),
        Err(BlockingError::Error(service_errors)) => match service_errors {
            RefreshErrors::Multiple(errors) => {
                HttpResponse::BadRequest().json(ErrorResponse { errors })
            }
            RefreshErrors::SessionNotFound => HttpResponse::NotFound().body("Not Found"),
            RefreshErrors::Unauthorized => HttpResponse::Unauthorized().body("Unauthorized"),
            RefreshErrors::UnexpectedError => http_500!(),
        },
        Err(BlockingError::Canceled) => http_500!(),
    }
}
