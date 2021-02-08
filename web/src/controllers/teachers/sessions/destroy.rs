use app::services::teachers::sessions::{sign_out, SignOutError};

use crate::prelude::*;

pub async fn handler(
  request: HttpRequest,
  web::Path(session_uuid): web::Path<String>,
  db_pool: web::Data<DbPool>,
) -> impl Responder {
  let db = db_connect!(db_pool);
  let refresh_token: String = require_refresh_token!(request);

  match web::block(move || sign_out(session_uuid, refresh_token, &db)).await {
    Ok(_) => http_200!(),
    Err(BlockingError::Error(service_errors)) => match service_errors {
      SignOutError::InvalidParams(errors) => http_400!(ErrorResponse { errors }),
      SignOutError::SessionNotFound => http_404!(),
      SignOutError::Unauthorized => http_401!(),
      SignOutError::UnexpectedError => http_500!(),
    },
    Err(BlockingError::Canceled) => http_500!(),
  }
}
