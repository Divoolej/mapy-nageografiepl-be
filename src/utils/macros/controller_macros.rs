#[macro_export]
macro_rules! http_401 {
  () => {{
    use crate::utils::responses::ErrorResponse;
    use actix_web::HttpResponse;

    HttpResponse::Unauthorized().json(ErrorResponse {
      errors: vec!["Unauthorized"],
    })
  }};
}

#[macro_export]
macro_rules! http_500 {
  () => {{
    use crate::utils::responses::ErrorResponse;
    use actix_web::HttpResponse;

    HttpResponse::InternalServerError().json(ErrorResponse {
      errors: vec!["Unexpected error has occurred"],
    })
  }};
}

#[macro_export]
macro_rules! require_refresh_token {
  ($req:ident) => {{
    use actix_web::http::header;
    // Check if header is present
    match $req.headers().get(header::AUTHORIZATION) {
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
