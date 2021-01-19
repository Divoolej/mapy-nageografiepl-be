use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse<'a> {
  pub errors: Vec<&'a str>,
}
