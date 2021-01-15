use diesel::backend::Backend;
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::result::Error;

use serde::{Serialize, Serializer};

use crate::handle_unexpected_err;
use crate::schema::sessions;
use crate::utils::{password, token};
use crate::models::Session;
use chrono::{DateTime, Utc, Duration};

// <RefreshErrors>
#[derive(Debug)]
pub enum RefreshErrors {
  UnexpectedError,
  Unauthorized,
  Multiple(Vec<RefreshError>),
}
// </RefreshError>

// <RefreshErrors>
#[derive(Debug)]
pub enum RefreshError {
  OwnerUuidIsBlank,
  RefreshTokenIsBlank,
}

impl ToString for RefreshError {
  fn to_string(&self) -> String {
    match self {
      Self::OwnerUuidIsBlank => String::from("Owner UUID can't be blank"),
      Self::RefreshTokenIsBlank => String::from("Refresh token can't be blank")
    }
  }
}

impl Serialize for RefreshError {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
    serializer.serialize_str(&self.to_string())
  }
}
// </RefreshError>

// <SessionValues>
#[derive(Insertable)]
#[table_name="sessions"]
struct SessionValues<'a> {
    owner_uuid: &'a str,
    access_token: String,
    access_token_expires_at: DateTime<Utc>,
}

// </SessionValues>

// <Refresh>
struct Refresh<'a> {
  owner_uuid: String,
  refresh_token: String,
  db: &'a PgConnection,
  session: Option<Session>,
}

impl<'a> Refresh<'a> {
  pub fn new(owner_uuid: String, refresh_token: String, db: &'a PgConnection) -> Self {
    Self {
      session: None,
      refresh_token,
      owner_uuid,
      db,
    }
  }

  pub fn call(self) -> Result<Session, RefreshErrors> {
    self.validate()?
        .get_session()?
        .update_session()?
        .finish()
  }

  fn validate(self) -> Result<Self, RefreshErrors> {
    let mut errors = vec![];

    if self.owner_uuid.trim().is_empty() { errors.push(RefreshError::OwnerUuidIsBlank); }
    if self.refresh_token.trim().is_empty() { errors.push(RefreshError::RefreshTokenIsBlank); }

    if errors.is_empty() {
      Ok(self)
    } else {
      Err(RefreshErrors::Multiple(errors))
    }
  }

  fn get_session(mut self) -> Result<Self, RefreshErrors> {
    use crate::schema::sessions::dsl::*;

    match sessions.filter(
      owner_type.eq("teacher")
          .and(owner_uuid.eq(&self.owner_uuid))
          .and(refresh_token.eq(&self.refresh_token))
        ).first::<Session>(self.db) {
      Ok(session) => {
        self.session = Some(session);
        Ok(self)
      },
      Err(Error::NotFound) => Err(RefreshErrors::Unauthorized),
      // Handle unexpected database-level errors:
      Err(error) => handle_unexpected_err!(error, RefreshErrors::UnexpectedError),
    }
  }

  fn update_session(mut self) -> Result<Self, RefreshErrors> {
    use crate::schema::sessions::dsl::*;

    match diesel::update(&self.session.unwrap())
        .set((
          access_token.eq(token::generate()),
          access_token_expires_at.eq(Utc::now() + Duration::days(1)),
        )).get_result::<Session>(self.db) {
      Ok(session) => {
        self.session = Some(session);
        Ok(self)
      }
      Err(error) => handle_unexpected_err!(error, RefreshErrors::UnexpectedError),
    }
  }

  fn finish(self) -> Result<Session, RefreshErrors> {
    // The unwrap is safe because we check for errors in update_session
    // and nothing else mutates the object in the meantime
    Ok(self.session.unwrap())
  }
}
// </Refresh>

pub fn refresh(owner_uuid: String, refresh_token: String, db: &PgConnection) -> Result<Session, RefreshErrors> {
  Refresh::new(owner_uuid, refresh_token, db).call()
}