#[macro_export]
macro_rules! db_connect {
  ($db:expr) => {
    match $db.get() {
      Ok(connection) => connection,
      Err(error) => {
        use crate::report_unexpected_err;
        use actix_web::HttpResponse;

        report_unexpected_err!(error);
        return HttpResponse::InternalServerError().body("Unexpected error has occurred");
      }
    };
  };
}
