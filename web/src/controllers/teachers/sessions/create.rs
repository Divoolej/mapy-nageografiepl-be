use app::services::teachers::sessions::{sign_in, SignInError};

use crate::prelude::*;
use crate::serializers::SessionSerializer;

#[derive(Deserialize)]
pub struct Params {
  email: String,
  password: String,
}

pub async fn handler(db_pool: web::Data<DbPool>, params: web::Json<Params>) -> impl Responder {
  let db = db_connect!(db_pool);
  let params = params.into_inner();

  match web::block(move || sign_in(params.email, params.password, &db)).await {
    Ok(session) => http_201!(SessionSerializer::from(&session)),
    Err(BlockingError::Error(service_errors)) => match service_errors {
      SignInError::InvalidParams(errors) => http_400!(ErrorResponse { errors }),
      SignInError::UnexpectedError => http_500!(),
    },
    Err(BlockingError::Canceled) => http_500!(),
  }
}
