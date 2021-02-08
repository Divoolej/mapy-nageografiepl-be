use app::services::teachers::sessions::{refresh, RefreshError};

use crate::prelude::*;
use crate::serializers::SessionSerializer;

pub async fn handler(
  request: HttpRequest,
  web::Path(session_uuid): web::Path<String>,
  db: web::Data<DbPool>,
) -> impl Responder {
  let conn = db_connect!(db);
  let refresh_token: String = require_refresh_token!(request);

  match web::block(move || refresh(session_uuid, refresh_token, &conn)).await {
    Ok(session) => http_200!(SessionSerializer::from(&session)),
    Err(BlockingError::Error(service_errors)) => match service_errors {
      RefreshError::InvalidParams(errors) => http_400!(ErrorResponse { errors }),
      RefreshError::SessionNotFound => http_404!(),
      RefreshError::Unauthorized => http_401!(),
      RefreshError::UnexpectedError => http_500!(),
    },
    Err(BlockingError::Canceled) => http_500!(),
  }
}
