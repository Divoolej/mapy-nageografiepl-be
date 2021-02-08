use serde::{Serialize, Serializer};
use db::prelude::*;
use db::models::{Session, Teacher};

use crate::utils::password;
use crate::{report_unexpected_err, handle_unexpected_err, make_serializable};

#[derive(PartialEq, Debug)]
pub enum ValidationError {
  EmailIsBlank,
  PasswordIsBlank,
  TeacherNotFound,
  PasswordDoesntMatch,
}

make_serializable!(ValidationError {
  EmailIsBlank => "Email can't be blank",
  PasswordIsBlank => "Password can't be blank",
  TeacherNotFound => "Invalid email/password combination",
  PasswordDoesntMatch => "Invalid email/password combination"
});

#[derive(PartialEq, Debug)]
pub enum SignInError {
  InvalidParams(Vec<ValidationError>),
  UnexpectedError,
}

struct SignIn<'a> {
  pub email: String,
  pub password: String,
  pub db: &'a DbConnection,
}

impl<'a> SignIn<'a> {
  fn new(email: String, password: String, db: &'a DbConnection) -> Self {
    Self {
      email,
      password,
      db,
    }
  }

  fn validate_params(&self) -> Result<(), SignInError> {
    let mut errors = vec![];

    if self.email.trim().is_empty() {
      errors.push(ValidationError::EmailIsBlank);
    }
    if self.password.trim().is_empty() {
      errors.push(ValidationError::PasswordIsBlank);
    }

    if errors.is_empty() {
      Ok(())
    } else {
      Err(SignInError::InvalidParams(errors))
    }
  }

  fn get_teacher(&self) -> Result<Teacher, SignInError> {
    let repository = TeachersRepository::new(self.db);

    match repository.find_by_email(&self.email) {
      Ok(teacher) => Ok(teacher),
      Err(DbError::RecordNotFound) => Err(
        SignInError::InvalidParams(
          vec![ValidationError::TeacherNotFound]
        )
      ),
      Err(error) => handle_unexpected_err!(error, SignInError::UnexpectedError),
    }
  }

  fn authenticate(&self, teacher: &Teacher) -> Result<(), SignInError> {
    match password::verify(&self.password, &teacher.password_digest) {
      Ok(true) => Ok(()),
      Ok(false) => Err(
        SignInError::InvalidParams(
          vec![ValidationError::PasswordDoesntMatch]
        )
      ),
      Err(error) => handle_unexpected_err!(error, SignInError::UnexpectedError),
    }
  }

  fn create_session(&self, teacher: &Teacher) -> Result<Session, SignInError> {
    let repository = SessionsRepository::new(self.db);

    match repository.create(teacher) {
      Ok(session) => Ok(session),
      Err(error) => handle_unexpected_err!(error, SignInError::UnexpectedError),
    }
  }

  fn call(self) -> Result<Session, SignInError> {
    self.validate_params()?;
    let teacher = self.get_teacher()?;
    self.authenticate(&teacher)?;
    let session = self.create_session(&teacher)?;

    Ok(session)
  }
}

pub fn sign_in(email: String, password: String, db: &DbConnection) -> Result<Session, SignInError> {
  SignIn::new(email, password, db).call()
}

#[cfg(test)]
mod tests {
  use serial_test::serial;
  use db::utils::test::with_db;
  use super::*;

  #[test]
  #[serial]
  fn sign_in_works() {
    with_db(|db| {
      let teachers_repository = TeachersRepository::new(&db);
      let sessions_repository = SessionsRepository::new(&db);
      let email = "john.doe@example.com".to_string();
      let password = "password".to_string();
      teachers_repository.create(email.clone(), password::digest(&password).unwrap()).unwrap();

      assert!(sign_in(email, password, &db).is_ok());
      assert_eq!(sessions_repository.count().unwrap(), 1);
    })
  }

  #[test]
  #[serial]
  fn sign_in_fails_when_email_is_blank() {
    with_db(|db| {
      let sessions_repository = SessionsRepository::new(&db);
      let email = "".to_string();
      let password = "password".to_string();

      assert_eq!(
        sign_in(email, password, &db),
        Err(SignInError::InvalidParams(vec![ValidationError::EmailIsBlank]))
      );
      assert_eq!(sessions_repository.count().unwrap(), 0);
    })
  }

  #[test]
  #[serial]
  fn sign_in_fails_when_password_is_blank() {
    with_db(|db| {
      let sessions_repository = SessionsRepository::new(&db);
      let email = "john.doe@example.com".to_string();
      let password = "".to_string();

      assert_eq!(
        sign_in(email, password, &db),
        Err(SignInError::InvalidParams(vec![ValidationError::PasswordIsBlank]))
      );
      assert_eq!(sessions_repository.count().unwrap(), 0);
    })
  }

  #[test]
  #[serial]
  fn sign_in_fails_when_teacher_doesnt_exist() {
    with_db(|db| {
      let sessions_repository = SessionsRepository::new(&db);
      let email = "john.doe@example.com".to_string();
      let password = "password".to_string();

      assert_eq!(
        sign_in(email, password, &db),
        Err(SignInError::InvalidParams(vec![ValidationError::TeacherNotFound]))
      );
      assert_eq!(sessions_repository.count().unwrap(), 0);
    })
  }

  #[test]
  #[serial]
  fn sign_in_fails_when_password_doesnt_match() {
    with_db(|db| {
      let sessions_repository = SessionsRepository::new(&db);
      let teachers_repository = TeachersRepository::new(&db);
      let email = "john.doe@example.com".to_string();
      let password = "password".to_string();
      let invalid_password = "invalid_password".to_string();
      teachers_repository.create(email.clone(), password::digest(&password).unwrap()).unwrap();

      assert_eq!(
        sign_in(email, invalid_password, &db),
        Err(SignInError::InvalidParams(vec![ValidationError::PasswordDoesntMatch]))
      );
      assert_eq!(sessions_repository.count().unwrap(), 0);
    })
  }
}
