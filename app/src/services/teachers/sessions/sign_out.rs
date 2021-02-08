use serde::{Serialize, Serializer};
use db::prelude::*;
use db::models::Session;

use crate::{report_unexpected_err, handle_unexpected_err, make_serializable};

#[derive(PartialEq, Debug)]
pub enum ValidationError {
  SessionUuidIsBlank,
  RefreshTokenIsBlank,
}

make_serializable!(ValidationError {
  SessionUuidIsBlank => "Session UUID can't be blank",
  RefreshTokenIsBlank => "Refresh token can't be blank"
});

#[derive(PartialEq, Debug)]
pub enum SignOutError {
  InvalidParams(Vec<ValidationError>),
  SessionNotFound,
  Unauthorized,
  UnexpectedError,
}

struct SignOut<'a> {
  session_uuid: String,
  refresh_token: String,
  sessions_repository: SessionsRepository<'a>,
}

impl<'a> SignOut<'a> {
  fn new(session_uuid: String, refresh_token: String, db: &'a DbConnection) -> Self {
    Self {
      sessions_repository: SessionsRepository::new(db),
      session_uuid,
      refresh_token,
    }
  }

  fn validate_params(&self) -> Result<(), SignOutError> {
    let mut errors = vec![];

    if self.session_uuid.trim().is_empty() {
      errors.push(ValidationError::SessionUuidIsBlank);
    }
    if self.refresh_token.trim().is_empty() {
      errors.push(ValidationError::RefreshTokenIsBlank);
    }

    if errors.is_empty() {
      Ok(())
    } else {
      Err(SignOutError::InvalidParams(errors))
    }
  }

  fn get_session(&self) -> Result<Session, SignOutError> {
    match self.sessions_repository.find_by_uuid(&self.session_uuid) {
      Ok(session) => Ok(session),
      Err(DbError::RecordNotFound) => Err(SignOutError::SessionNotFound),
      Err(error) => handle_unexpected_err!(error, SignOutError::UnexpectedError),
    }
  }

  fn authorize(&self, session: &Session) -> Result<(), SignOutError> {
    if session.refresh_token == self.refresh_token {
      Ok(())
    } else {
      Err(SignOutError::Unauthorized)
    }
  }

  fn destroy_session(&self, session: &Session) -> Result<(), SignOutError> {
    match self.sessions_repository.destroy(session) {
      Ok(_) => Ok(()),
      Err(error) => handle_unexpected_err!(error, SignOutError::UnexpectedError),
    }
  }

  fn call(self) -> Result<(), SignOutError> {
    self.validate_params()?;
    let session = self.get_session()?;
    self.authorize(&session)?;
    self.destroy_session(&session)?;

    Ok(())
  }
}

pub fn sign_out(session_uuid: String, refresh_token: String, db: &DbConnection) -> Result<(), SignOutError> {
  SignOut::new(session_uuid, refresh_token, db).call()
}

#[cfg(test)]
mod tests {
  use serial_test::serial;
  use db::utils::test::with_db;
  use super::*;

  #[test]
  #[serial]
  fn sign_out_works() {
    with_db(|db| {
      let teachers_repository = TeachersRepository::new(&db);
      let sessions_repository = SessionsRepository::new(&db);
      let teacher = teachers_repository.create("john.doe@example.com".into(), "test".into()).unwrap();
      let session = sessions_repository.create(&teacher).unwrap();

      let result = sign_out(session.uuid, session.refresh_token, &db);
      assert!(result.is_ok());
      assert_eq!(sessions_repository.count().unwrap(), 0);
    });
  }

  #[test]
  #[serial]
  fn sign_out_fails_when_uuid_blank() {
    with_db(|db| {
      let teachers_repository = TeachersRepository::new(&db);
      let sessions_repository = SessionsRepository::new(&db);
      let teacher = teachers_repository.create("john.doe@example.com".into(), "test".into()).unwrap();
      let session = sessions_repository.create(&teacher).unwrap();

      assert_eq!(
        sign_out("".into(), session.refresh_token, &db),
        Err(SignOutError::InvalidParams(vec![ValidationError::SessionUuidIsBlank])),
      );
      assert_eq!(sessions_repository.count().unwrap(), 1);
    });
  }

  #[test]
  #[serial]
  fn sign_out_fails_when_refresh_token_blank() {
    with_db(|db| {
      let teachers_repository = TeachersRepository::new(&db);
      let sessions_repository = SessionsRepository::new(&db);
      let teacher = teachers_repository.create("john.doe@example.com".into(), "test".into()).unwrap();
      let session = sessions_repository.create(&teacher).unwrap();

      assert_eq!(
        sign_out(session.uuid, "".into(), &db),
        Err(SignOutError::InvalidParams(vec![ValidationError::RefreshTokenIsBlank])),
      );
      assert_eq!(sessions_repository.count().unwrap(), 1);
    });
  }

  #[test]
  #[serial]
  fn sign_out_fails_when_session_doesnt_exist() {
    with_db(|db| {
      assert_eq!(
        sign_out("uuid".into(), "refresh_token".into(), &db),
        Err(SignOutError::SessionNotFound),
      );
    });
  }

  #[test]
  #[serial]
  fn sign_out_fails_when_refresh_token_doesnt_match() {
    with_db(|db| {
      let teachers_repository = TeachersRepository::new(&db);
      let sessions_repository = SessionsRepository::new(&db);
      let teacher = teachers_repository.create("john.doe@example.com".into(), "test".into()).unwrap();
      let session = sessions_repository.create(&teacher).unwrap();

      assert_eq!(
        sign_out(session.uuid, "invalid_refresh_token".into(), &db),
        Err(SignOutError::Unauthorized),
      );
      assert_eq!(sessions_repository.count().unwrap(), 1);
    });
  }
}
