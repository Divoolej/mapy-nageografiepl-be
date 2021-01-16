use diesel::{prelude::*, result::Error};

use crate::models::Session;
use crate::prelude::*;

// <DestroyErrors>
#[derive(Debug)]
pub enum DestroyErrors {
  UnexpectedError,
  SessionNotFound,
  Unauthorized,
  Multiple(Vec<DestroyError>),
}
// </DestroyErrors>

// <DestroyError>
#[derive(Debug)]
pub enum DestroyError {
  SessionUuidIsBlank,
  RefreshTokenIsBlank,
}

make_serializable!(DestroyError {
  SessionUuidIsBlank => "Session UUID can't be blank",
  RefreshTokenIsBlank => "Refresh token can't be blank"
});
// </DestroyError>

// <Refresh>
struct Refresh<'a> {
  session_uuid: String,
  refresh_token: String,
  db: &'a PgConnection,
  session: Option<Session>,
}

impl<'a> Refresh<'a> {
  pub fn new(session_uuid: String, refresh_token: String, db: &'a PgConnection) -> Self {
    Self {
      session: None,
      refresh_token,
      session_uuid,
      db,
    }
  }

  pub fn call(self) -> Result<(), DestroyErrors> {
    self
      .validate()?
      .get_session()?
      .authenticate()?
      .delete_session()?
      .finish()
  }

  fn validate(self) -> Result<Self, DestroyErrors> {
    let mut errors = vec![];

    if self.session_uuid.trim().is_empty() {
      errors.push(DestroyError::SessionUuidIsBlank);
    }
    if self.refresh_token.trim().is_empty() {
      errors.push(DestroyError::RefreshTokenIsBlank);
    }

    if errors.is_empty() {
      Ok(self)
    } else {
      Err(DestroyErrors::Multiple(errors))
    }
  }

  fn get_session(mut self) -> Result<Self, DestroyErrors> {
    use crate::schema::sessions::dsl::*;

    match sessions
      .filter(owner_type.eq("teacher").and(uuid.eq(&self.session_uuid)))
      .first::<Session>(self.db)
    {
      Ok(session) => {
        self.session = Some(session);
        Ok(self)
      }
      Err(Error::NotFound) => Err(DestroyErrors::SessionNotFound),
      // Handle unexpected database-level errors:
      Err(error) => handle_unexpected_err!(error, DestroyErrors::UnexpectedError),
    }
  }

  fn authenticate(self) -> Result<Self, DestroyErrors> {
    // This unwrap is safe as we ensure session presence in #get_session
    if self.session.as_ref().unwrap().refresh_token == self.refresh_token {
      Ok(self)
    } else {
      Err(DestroyErrors::Unauthorized)
    }
  }

  fn delete_session(mut self) -> Result<Self, DestroyErrors> {
    // This unwrap is safe as we ensure session presence in #get_session
    match diesel::delete(&self.session.unwrap()).execute(self.db) {
      Ok(1) => {
        self.session = None;
        Ok(self)
      }
      Ok(_) => unreachable!("Record was deleted but rows affected != 1!"),
      Err(error) => handle_unexpected_err!(error, DestroyErrors::UnexpectedError),
    }
  }

  fn finish(self) -> Result<(), DestroyErrors> {
    // The unwrap is safe because we check for errors in update_session
    // and nothing else mutates the object in the meantime
    Ok(())
  }
}
// </Refresh>

pub fn destroy(owner_uuid: String, refresh_token: String, db: &PgConnection) -> Result<(), DestroyErrors> {
  Refresh::new(owner_uuid, refresh_token, db).call()
}
