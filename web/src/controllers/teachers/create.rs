use app::services::teachers::{sign_up, SignUpError};

use crate::prelude::*;

#[derive(Deserialize)]
pub struct Params {
  email: String,
  password: String,
}

pub async fn handler(db_pool: web::Data<DbPool>, params: web::Json<Params>) -> impl Responder {
  let db = db_connect!(db_pool);
  let params = params.into_inner();

  match web::block(move || sign_up(params.email, params.password, &db)).await {
    Ok(_) => http_201!(),
    Err(BlockingError::Error(service_errors)) => match service_errors {
      SignUpError::InvalidParams(errors) => http_400!(ErrorResponse { errors }),
      SignUpError::UnexpectedError => http_500!(),
    },
    Err(BlockingError::Canceled) => http_500!(),
  }
}

