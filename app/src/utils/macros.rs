#[macro_export]
macro_rules! report_unexpected_err {
  ($err:expr) => {{
    use log::error;
    #[cfg(not(test))] use rollbar::report_error;
    #[cfg(not(test))] use crate::prelude::ROLLBAR_CLIENT;

    let err = $err; // "report_error!" doesn't accept "$err" so we need to bind it first.
    error!("{:?}", err); // Log error first, as "report_error!" consumes the binding
    #[cfg(not(test))] report_error!(ROLLBAR_CLIENT, err); // Start a thread for uploading the error to Rollbar
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
  ($err_type:ty { $($err_variant:ident => $err_description:expr),+ $(,)? }) => {
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
