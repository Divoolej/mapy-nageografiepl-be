#[macro_use]
extern crate lazy_static;

pub mod utils;
pub mod services;

pub mod prelude {
  pub use crate::utils::constants::ROLLBAR_CLIENT;
  pub use crate::{handle_unexpected_err, report_unexpected_err};
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
