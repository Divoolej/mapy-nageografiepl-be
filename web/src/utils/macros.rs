#[macro_export]
macro_rules! http_200 {
  () => {
    HttpResponse::Ok().json(crate::utils::responses::EmptyResponse {})
  };
  ($body:expr) => {
    HttpResponse::Ok().json($body)
  };
}

#[macro_export]
macro_rules! http_201 {
  () => {
    HttpResponse::Created().json(crate::utils::responses::EmptyResponse {})
  };
  ($body:expr) => {
    HttpResponse::Created().json($body)
  };
}

#[macro_export]
macro_rules! http_400 {
  ($body:expr) => {
    HttpResponse::BadRequest().json($body)
  };
}

#[macro_export]
macro_rules! http_401 {
  () => {{
    use actix_web::HttpResponse;
    use crate::utils::responses::ErrorResponse;

    HttpResponse::Unauthorized().json(ErrorResponse {
      errors: vec!["Unauthorized"],
    })
  }};
}

#[macro_export]
macro_rules! http_404 {
  () => {{
    use actix_web::HttpResponse;
    use crate::utils::responses::ErrorResponse;

    HttpResponse::NotFound().json(ErrorResponse {
      errors: vec!["Not found"],
    })
  }};
}

#[macro_export]
macro_rules! http_500 {
  () => {{
    use actix_web::HttpResponse;
    use crate::utils::responses::ErrorResponse;

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
          // Extract the token from the header
          if let Some(value) = value.split("Bearer ").nth(1) {
            // Check if the string is not empty
            if value.trim().is_empty() {
              return http_401!();
            } else {
              value.trim().to_string()
            }
          } else {
            return http_401!();
          }
        }
        Err(_) => return http_401!(),
      },
      None => return http_401!(),
    }
  }};
}

#[macro_export]
macro_rules! db_connect {
  ($db:expr) => {
    match $db.get() {
      Ok(connection) => connection,
      Err(error) => {
        use actix_web::HttpResponse;
        use app::prelude::report_unexpected_err;

        report_unexpected_err!(error);
        return HttpResponse::InternalServerError().body("Unexpected error has occurred");
      }
    };
  };
}
