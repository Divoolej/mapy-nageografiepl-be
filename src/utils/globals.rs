lazy_static! {
  pub static ref ROLLBAR_CLIENT: rollbar::Client = {
    rollbar::Client::new(
      std::env::var("ROLLBAR_ACCESS_TOKEN").expect("ROLLBAR_ACCESS_TOKEN is not set"),
      std::env::var("ROLLBAR_ENVIRONMENT").expect("ROLLBAR_ENVIRONMENT is not set"),
    )
  };
}
