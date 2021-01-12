use diesel::prelude::*;
use diesel::result::Error;
use log::error;
use serde::{Serialize, Serializer};

use crate::schema::sessions;
use crate::utils::{password, token};
use crate::models::{Session, Teacher};
use chrono::{DateTime, Utc, Duration};

// <CreateError>
#[derive(Debug)]
pub enum CreateError {
  EmailIsBlank,
  EmailNotFound,
  PasswordIsBlank,
  PasswordDoesntMatch,
}

impl ToString for CreateError {
  fn to_string(&self) -> String {
    match self {
      Self::EmailIsBlank => String::from("Email can't be blank"),
      Self::PasswordIsBlank => String::from("Password can't be blank"),
      Self::EmailNotFound | Self::PasswordDoesntMatch => String::from("Invalid email/password combination"),
    }
  }
}

impl Serialize for CreateError {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
    serializer.serialize_str(&self.to_string())
  }
}
// </CreateError>

// <SessionValues>
#[derive(Insertable)]
#[table_name="sessions"]
struct SessionValues<'a> {
    owner_type: String,
    owner_uuid: &'a str,
    access_token: String,
    access_token_expires_at: DateTime<Utc>,
    refresh_token: String,
    refresh_token_expires_at: DateTime<Utc>,
}

impl<'a> SessionValues<'a> {
  fn new(teacher_uuid: &'a str) -> Self {
    Self {
      owner_type: String::from("teacher"),
      owner_uuid: teacher_uuid,
      access_token: token::generate(),
      refresh_token: token::generate(),
      access_token_expires_at: Utc::now() + Duration::days(1),
      refresh_token_expires_at: Utc::now() + Duration::weeks(4),
    }
  }
}
// </SessionValues>

// <Create>
struct Create<'a> {
  email: String,
  password: String,
  db: &'a PgConnection,
  teacher: Option<Teacher>,
  session: Option<Session>,
}

impl<'a> Create<'a> {
  pub fn new(email: String, password: String, db: &'a PgConnection) -> Self {
    Self {
      email,
      password,
      db,
      teacher: None,
      session: None,
    }
  }

  pub fn call(self) -> Result<Session, Option<Vec<CreateError>>> {
    self.validate()?
        .get_teacher()?
        .authenticate()?
        .insert_session()?
        .finish()
  }

  fn validate(self) -> Result<Self, Option<Vec<CreateError>>> {
    let mut errors = vec![];

    if self.email.trim().is_empty() { errors.push(CreateError::EmailIsBlank); }
    if self.password.trim().is_empty() { errors.push(CreateError::PasswordIsBlank); }

    if errors.is_empty() {
      Ok(self)
    } else {
      Err(Some(errors))
    }
  }

  fn get_teacher(mut self) -> Result<Self, Option<Vec<CreateError>>> {
    use crate::schema::teachers::dsl::*;

    match teachers
        .filter(email.eq(&self.email))
        .first::<Teacher>(self.db) {
      Ok(teacher) => {
        self.teacher = Some(teacher);
        Ok(self)
      },
      Err(err) => match err {
        Error::NotFound => Err(Some(vec![CreateError::EmailNotFound])),
        // Handle unexpected database-level errors:
        error => {
          error!("{}", error);
          Err(None)
        },
      }
    }
  }

  fn authenticate(self) -> Result<Self, Option<Vec<CreateError>>> {
    // It's safe to unwrap the teacher because we ensure it's presence in #get_teacher method
    match password::verify(&self.password, &self.teacher.as_ref().unwrap().password_digest) {
      Ok(true) => Ok(self),
      Ok(false) => Err(Some(vec![CreateError::PasswordDoesntMatch])),
      // Handle unexpected errors from argon2:
      Err(error) => {
        error!("{}", error);
        Err(None)
      },
    }
  }

  fn insert_session(mut self) -> Result<Self, Option<Vec<CreateError>>> {
    // It's safe to unwrap the teacher because we ensure it's presence in #get_teacher method
    let values = SessionValues::new(&self.teacher.as_ref().unwrap().uuid);

    match diesel::insert_into(sessions::table)
        .values(&values)
        .get_result::<Session>(self.db) {
      Ok(session) => {
        self.session = Some(session);
        Ok(self)
      },
      // Handle unexpected database-level errors:
      Err(error) => {
        error!("{}", error);
        Err(None)
      },
    }
  }

  fn finish(self) -> Result<Session, Option<Vec<CreateError>>> {
    self.session.ok_or(None)
  }
}
// </Create>

pub fn create(email: String, password: String, db: &PgConnection) -> Result<Session, Option<Vec<CreateError>>> {
  Create::new(email, password, db).call()
}