use rollbar;

lazy_static! {
    pub static ref ROLLBAR_CLIENT: rollbar::Client = {
        rollbar::Client::new(
            std::env::var("ROLLBAR_ACCESS_TOKEN").expect("ROLLBAR_ACCESS_TOKEN is not set"),
            std::env::var("ROLLBAR_ENVIRONMENT").expect("ROLLBAR_ENVIRONMENT is not set"),
        )
    };
}

#[macro_export]
macro_rules! report_unexpected_err {
    ($err:expr) => {{
        use log::error; // In case "error!" is not in scope
        use crate::utils::errors::ROLLBAR_CLIENT; // In case ROLLBAR_CLIENT is not in scope
        let err = $err; // "report_error!" doesn't accept "$err" so we need to bind it first.
        error!("{:?}", err); // Log error first, as "report_error!" consumes the binding
        report_error!(ROLLBAR_CLIENT, err); // Start a thread for uploading the error to Rollbar
      }};
}

#[macro_export]
macro_rules! handle_unexpected_err {
    ($err:expr, $result:expr) => {{
        use crate::report_unexpected_err;
        report_unexpected_err!($err);
        Err($result) // Bubble the error
    }};
}

#[macro_export]
macro_rules! make_serializable {
  ($err_type:ty { $($err_variant:ident => $err_description:expr),+ $(,)?}) => {
    impl ToString for $err_type {
      fn to_string(&self) -> String {
        match self {
          $( Self::$err_variant => String::from($err_description), )+
        }
      }
    }

    impl Serialize for $err_type {
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(&self.to_string())
      }
    }
  }
}
