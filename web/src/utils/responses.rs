use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse<T: Serialize> {
  pub errors: Vec<T>,
}

#[derive(Serialize)]
pub struct EmptyResponse {}
