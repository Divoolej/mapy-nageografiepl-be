#[macro_export]
macro_rules! db_connect {
  ($db:expr) => {
    match $db.get() {
      Ok(connection) => connection,
      Err(error) => {
        use actix_web::HttpResponse;
        use crate::report_unexpected_err;

        report_unexpected_err!(error);
        return HttpResponse::InternalServerError().body("Unexpected error has occurred");
      }
    };
  }
}