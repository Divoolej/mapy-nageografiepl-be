#[macro_export]
macro_rules! http_500 {
  () => {
    HttpResponse::InternalServerError().body("Unexpected error has occurred")
  };
}