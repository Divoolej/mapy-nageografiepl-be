use serde::{Serialize, Serializer};
use db::prelude::*;

use crate::utils::{
  password,
  constants::{MIN_PASSWORD_LENGTH, MAX_PASSWORD_LENGTH, EMAIL_REGEX},
};
use crate::{report_unexpected_err, handle_unexpected_err, make_serializable};

#[derive(PartialEq, Debug)]
pub enum ValidationError {
  EmailIsBlank,
  EmailIsInvalid,
  PasswordIsBlank,
  PasswordIsTooShort,
  PasswordIsTooLong,
}

make_serializable!(ValidationError {
  EmailIsBlank => "Email can't be blank",
  EmailIsInvalid => "Email is invalid",
  PasswordIsBlank => "Password can't be blank",
  // TODO: Give better feedback on password security
  PasswordIsTooShort => "Password is too short (minimum is 8 characters)",
  PasswordIsTooLong => "Password is too long (maximum is 128 characters)",
});

#[derive(PartialEq, Debug)]
pub enum SignUpError {
  InvalidParams(Vec<ValidationError>),
  UnexpectedError,
}

struct SignUp<'a> {
  pub email: String,
  pub password: String,
  pub db: &'a DbConnection,
}

impl<'a> SignUp<'a> {
  fn new(email: String, password: String, db: &'a DbConnection) -> Self {
    Self {
      email,
      password,
      db,
    }
  }

  fn validate_params(&self) -> Result<(), SignUpError> {
    let mut errors = vec![];

    if self.email.trim().is_empty() {
      errors.push(ValidationError::EmailIsBlank);
    } else if !EMAIL_REGEX.is_match(&self.email) {
      errors.push(ValidationError::EmailIsInvalid);
    }

    let password = self.password.trim();
    if password.is_empty() {
      errors.push(ValidationError::PasswordIsBlank);
    } else if password.len() > MAX_PASSWORD_LENGTH {
      // TODO: Implement better password validation (maybe using zxcvbn-rs)
      errors.push(ValidationError::PasswordIsTooLong);
    } else if password.len() < MIN_PASSWORD_LENGTH {
      // TODO: Implement better password validation (maybe using zxcvbn-rs)
      errors.push(ValidationError::PasswordIsTooShort);
    }

    if errors.is_empty() {
      Ok(())
    } else {
      Err(SignUpError::InvalidParams(errors))
    }
  }

  fn create_teacher(&self) -> Result<(), SignUpError> {
    let repository = TeachersRepository::new(self.db);
    let password_digest = password::digest(&self.password)
      .map_err(|error| {
        // Report unexpected errors from argon2
        report_unexpected_err!(error);
        SignUpError::UnexpectedError
      })?;

    match repository.create(self.email.clone(), password_digest) {
      Ok(_) => Ok(()),
      // If the email is already taken we still want to pretend that the sign up
      // was successful - this is a security measure against user enumeration
      // https://blog.rapid7.com/2017/06/15/about-user-enumeration
      Err(DbError::UniqueConstraintViolation(_)) => Ok(()),
      Err(error) => handle_unexpected_err!(error, SignUpError::UnexpectedError),
    }
  }

  fn call(self) -> Result<(), SignUpError> {
    self.validate_params()?;
    self.create_teacher()?;

    Ok(())
  }
}

pub fn sign_up(email: String, password: String, db: &DbConnection) -> Result<(), SignUpError> {
  SignUp::new(email, password, db).call()
}

#[cfg(test)]
mod tests {
  use serial_test::serial;
  use db::utils::test::with_db;
  use super::*;

  #[test]
  #[serial]
  fn sign_up_works() {
    with_db(|db| {
      let repository = TeachersRepository::new(&db);
      let email = "john.doe@example.com".to_string();
      let password = "password".to_string();

      assert!(sign_up(email, password, &db).is_ok());
      assert_eq!(repository.count().unwrap(), 1);
    })
  }

  #[test]
  #[serial]
  fn sign_up_works_when_user_already_exists() {
    with_db(|db| {
      let repository = TeachersRepository::new(&db);
      let email = "john.doe@example.com".to_string();
      let password = "password".to_string();
      repository.create(email.clone(), "test".into()).unwrap();

      assert!(sign_up(email, password, &db).is_ok());
      assert_eq!(repository.count().unwrap(), 1);
    })
  }

  #[test]
  #[serial]
  fn sign_up_fails_when_email_is_blank() {
    with_db(|db| {
      let repository = TeachersRepository::new(&db);
      let email = "".to_string();
      let password = "password".to_string();

      assert_eq!(
        sign_up(email, password, &db),
        Err(SignUpError::InvalidParams(vec![ValidationError::EmailIsBlank]))
      );
      assert_eq!(repository.count().unwrap(), 0);
    })
  }

  #[test]
  #[serial]
  fn sign_up_fails_when_email_is_invalid() {
    with_db(|db| {
      let repository = TeachersRepository::new(&db);
      let email = "john.doe".to_string();
      let password = "password".to_string();

      assert_eq!(
        sign_up(email, password, &db),
        Err(SignUpError::InvalidParams(vec![ValidationError::EmailIsInvalid]))
      );
      assert_eq!(repository.count().unwrap(), 0);
    })
  }

  #[test]
  #[serial]
  fn sign_up_fails_when_password_is_blank() {
    with_db(|db| {
      let repository = TeachersRepository::new(&db);
      let email = "john.doe@example.com".to_string();
      let password = "".to_string();

      assert_eq!(
        sign_up(email, password, &db),
        Err(SignUpError::InvalidParams(vec![ValidationError::PasswordIsBlank]))
      );
      assert_eq!(repository.count().unwrap(), 0);
    })
  }

  #[test]
  #[serial]
  fn sign_up_fails_when_password_is_too_short() {
    with_db(|db| {
      let repository = TeachersRepository::new(&db);
      let email = "john.doe@example.com".to_string();
      let password = "qwe".to_string();

      assert_eq!(
        sign_up(email, password, &db),
        Err(SignUpError::InvalidParams(vec![ValidationError::PasswordIsTooShort]))
      );
      assert_eq!(repository.count().unwrap(), 0);
    })
  }

  #[test]
  #[serial]
  fn sign_up_fails_when_password_is_too_long() {
    with_db(|db| {
      let repository = TeachersRepository::new(&db);
      let email = "john.doe@example.com".to_string();
      let password: String = ['a'; 129].iter().collect();

      assert_eq!(
        sign_up(email, password, &db),
        Err(SignUpError::InvalidParams(vec![ValidationError::PasswordIsTooLong]))
      );
      assert_eq!(repository.count().unwrap(), 0);
    })
  }
}
