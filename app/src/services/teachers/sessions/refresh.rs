use chrono::{Duration, Utc};
use serde::{Serialize, Serializer};
use db::prelude::*;
use db::models::Session;
use db::utils::token;

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
pub enum RefreshError {
  InvalidParams(Vec<ValidationError>),
  SessionNotFound,
  Unauthorized,
  UnexpectedError,
}

struct Refresh<'a> {
  session_uuid: String,
  refresh_token: String,
  sessions_repository: SessionsRepository<'a>,
}

impl<'a> Refresh<'a> {
  fn new(session_uuid: String, refresh_token: String, db: &'a DbConnection) -> Self {
    Self {
      sessions_repository: SessionsRepository::new(db),
      session_uuid,
      refresh_token,
    }
  }

  fn validate_params(&self) -> Result<(), RefreshError> {
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
      Err(RefreshError::InvalidParams(errors))
    }
  }

  fn get_session(&self) -> Result<Session, RefreshError> {
    match self.sessions_repository.find_by_uuid(&self.session_uuid) {
      Ok(session) => Ok(session),
      Err(DbError::RecordNotFound) => Err(RefreshError::SessionNotFound),
      Err(error) => handle_unexpected_err!(error, RefreshError::UnexpectedError),
    }
  }

  fn authorize(&self, session: &Session) -> Result<(), RefreshError> {
    if session.refresh_token == self.refresh_token {
      Ok(())
    } else {
      Err(RefreshError::Unauthorized)
    }
  }

  fn update_session(&self, session: &mut Session) -> Result<(), RefreshError> {
    session.access_token = token::generate();
    session.access_token_expires_at = Utc::now() + Duration::days(1);

    match self.sessions_repository.save(session) {
      Ok(_) => Ok(()),
      Err(error) => handle_unexpected_err!(error, RefreshError::UnexpectedError),
    }
  }

  fn call(self) -> Result<Session, RefreshError> {
    self.validate_params()?;
    let mut session = self.get_session()?;
    self.authorize(&session)?;
    self.update_session(&mut session)?;

    Ok(session)
  }
}

pub fn refresh(session_uuid: String, refresh_token: String, db: &DbConnection) -> Result<Session, RefreshError> {
  Refresh::new(session_uuid, refresh_token, db).call()
}

#[cfg(test)]
mod tests {
  use serial_test::serial;
  use db::utils::test::with_db;
  use super::*;

  #[test]
  #[serial]
  fn refresh_works() {
    with_db(|db| {
      let teachers_repository = TeachersRepository::new(&db);
      let sessions_repository = SessionsRepository::new(&db);
      let teacher = teachers_repository.create("john.doe@example.com".into(), "test".into()).unwrap();
      let session = sessions_repository.create(&teacher).unwrap();

      let result = refresh(session.uuid, session.refresh_token, &db);
      assert!(result.is_ok());
      assert!(result.unwrap().access_token != session.access_token);
    });
  }

  #[test]
  #[serial]
  fn refresh_fails_when_uuid_blank() {
    with_db(|db| {
      let teachers_repository = TeachersRepository::new(&db);
      let sessions_repository = SessionsRepository::new(&db);
      let teacher = teachers_repository.create("john.doe@example.com".into(), "test".into()).unwrap();
      let session = sessions_repository.create(&teacher).unwrap();

      assert_eq!(
        refresh("".into(), session.refresh_token, &db),
        Err(RefreshError::InvalidParams(vec![ValidationError::SessionUuidIsBlank])),
      );
    });
  }

  #[test]
  #[serial]
  fn refresh_fails_when_refresh_token_blank() {
    with_db(|db| {
      let teachers_repository = TeachersRepository::new(&db);
      let sessions_repository = SessionsRepository::new(&db);
      let teacher = teachers_repository.create("john.doe@example.com".into(), "test".into()).unwrap();
      let session = sessions_repository.create(&teacher).unwrap();

      assert_eq!(
        refresh(session.uuid, "".into(), &db),
        Err(RefreshError::InvalidParams(vec![ValidationError::RefreshTokenIsBlank])),
      );
    });
  }

  #[test]
  #[serial]
  fn refresh_fails_when_session_doesnt_exist() {
    with_db(|db| {
      assert_eq!(
        refresh("uuid".into(), "refresh_token".into(), &db),
        Err(RefreshError::SessionNotFound),
      );
    });
  }

  #[test]
  #[serial]
  fn refresh_fails_when_refresh_token_doesnt_match() {
    with_db(|db| {
      let teachers_repository = TeachersRepository::new(&db);
      let sessions_repository = SessionsRepository::new(&db);
      let teacher = teachers_repository.create("john.doe@example.com".into(), "test".into()).unwrap();
      let session = sessions_repository.create(&teacher).unwrap();

      assert_eq!(
        refresh(session.uuid, "invalid_refresh_token".into(), &db),
        Err(RefreshError::Unauthorized),
      );
    });
  }
}
