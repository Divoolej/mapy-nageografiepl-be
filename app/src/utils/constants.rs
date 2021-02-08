use regex::Regex;
use rollbar;

lazy_static! {
  pub static ref ROLLBAR_CLIENT: rollbar::Client = {
    rollbar::Client::new(
      std::env::var("ROLLBAR_ACCESS_TOKEN").expect("ROLLBAR_ACCESS_TOKEN is not set"),
      std::env::var("ROLLBAR_ENVIRONMENT").expect("ROLLBAR_ENVIRONMENT is not set"),
    )
  };
}

lazy_static! {
  pub static ref EMAIL_REGEX: Regex =
    // Email regex taken from the infamous https://stackoverflow.com/questions/201323/how-to-validate-an-email-address-using-a-regular-expression
    Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$")
      .expect("Failed to parse regular expression!");
}

pub const MIN_PASSWORD_LENGTH: usize = 8;
pub const MAX_PASSWORD_LENGTH: usize = 128;
