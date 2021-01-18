use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse<'a> {
  pub errors: Vec<&'a str>,
}

#[macro_export]
macro_rules! http_500 {
  () => {
    HttpResponse::InternalServerError().json(crate::utils::http::ErrorResponse {
      errors: vec!["Unexpected error has occurred"],
    })
  };
}

#[macro_export]
macro_rules! http_401 {
  () => {
    HttpResponse::Unauthorized().json(crate::utils::http::ErrorResponse {
      errors: vec!["Unauthorized"],
    })
  };
}

#[macro_export]
macro_rules! require_refresh_token {
  ($req:ident) => {{
    // Check if header is present
    match $req.headers().get(actix_web::http::header::AUTHORIZATION) {
      // Check if header value is a valid string
      Some(header_value) => match header_value.to_str() {
        Ok(value) => {
          // Check if the string is not empty
          if value.trim().is_empty() {
            return http_401!();
          } else {
            value.trim().to_string()
          }
        }
        Err(_) => return http_401!(),
      },
      None => return http_401!(),
    }
  }};
}
