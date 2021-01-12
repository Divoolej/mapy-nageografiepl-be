use diesel::prelude::*;
use diesel::result::{Error, DatabaseErrorKind};
use log::error;
use regex::Regex;
use serde::{Serialize, Serializer};

use crate::schema::teachers;
use crate::utils::password;
use crate::models::Teacher;
use uuid::Uuid;

// <CreateError>
#[derive(Debug)]
pub enum CreateError {
  EmailIsBlank,
  EmailIsInvalid,
  PasswordIsBlank,
  PasswordIsTooShort,
}

impl ToString for CreateError {
  fn to_string(&self) -> String {
    match self {
      Self::EmailIsBlank => String::from("Email can't be blank"),
      Self::EmailIsInvalid => String::from("Email is invalid"),
      Self::PasswordIsBlank => String::from("Password can't be blank"),
      // TODO: Give better feedback on password security
      Self::PasswordIsTooShort => String::from("Password is too short (minimum is 8 characters)"),
    }
  }
}

impl Serialize for CreateError {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
    serializer.serialize_str(&self.to_string())
  }
}
// </CreateError>

// <TeacherValues>
#[derive(Insertable)]
#[table_name="teachers"]
struct TeacherValues<'a> {
  email: &'a str,
  uuid: String,
  password_digest: String,
}

impl<'a> TeacherValues<'a> {
  pub fn new(email: &'a str, password_digest: String) -> Self {
    Self {
      uuid: Uuid::new_v4().to_string(),
      password_digest,
      email,
    }
  }
}
// </TeacherValues>

// <Create>
struct Create<'a> {
  email: String,
  password: String,
  db: &'a PgConnection,
}

impl<'a> Create<'a> {
  pub fn new(email: String, password: String, db: &'a PgConnection) -> Self {
    Self {
      email,
      password,
      db,
    }
  }

  pub fn call(self) -> Result<(), Option<Vec<CreateError>>> {
    self.validate()?
        .insert_teacher()?
        .finish()
  }

  fn validate(self) -> Result<Self, Option<Vec<CreateError>>> {
    let mut errors = vec![];

    // Email regex taken from the infamous https://stackoverflow.com/questions/201323/how-to-validate-an-email-address-using-a-regular-expression
    let email_regex = Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$").unwrap();
    if self.email.trim().is_empty() {
      errors.push(CreateError::EmailIsBlank);
    } else if !email_regex.is_match(&self.email) {
      errors.push(CreateError::EmailIsInvalid);
    }

    if self.password.trim().is_empty() {
      errors.push(CreateError::PasswordIsBlank);
    } else if self.password.trim().len() < 8 {
      // TODO: Implement better password validation (maybe using zxcvbn-rs)
      errors.push(CreateError::PasswordIsTooShort);
    }

    if errors.is_empty() {
      Ok(self)
    } else {
      Err(Some(errors))
    }
  }

  fn insert_teacher(self) -> Result<Self, Option<Vec<CreateError>>> {
    let values = TeacherValues::new(
      &self.email,
      password::digest(&self.password)
          .map_err(|error| {
            // Handle unexpected errors from argon2:
            error!("{}", error);
            None
          })?,
    );

    match diesel::insert_into(teachers::table)
        .values(&values)
        .get_result::<Teacher>(self.db) {
      Ok(_) => Ok(self),
      Err(err) => match err {
        // If the email is already taken we still want to pretend that the sign up
        // was successful - this is a security measure against email enumeration
        // https://blog.rapid7.com/2017/06/15/about-user-enumeration
        Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => Ok(self),
        // Handle unexpected database-level errors:
        error => {
          error!("{}", error);
          Err(None)
        },
      }
    }
  }

  fn finish(self) -> Result<(), Option<Vec<CreateError>>> { Ok(()) }
}
// </Create>

pub fn create(email: String, password: String, db: &PgConnection) -> Result<(), Option<Vec<CreateError>>> {
  Create::new(email, password, db).call()
}