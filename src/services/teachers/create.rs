use diesel::{
  prelude::*,
  result::{DatabaseErrorKind, Error},
};
use regex::Regex;
use uuid::Uuid;

use crate::models::Teacher;
use crate::prelude::*;
use crate::schema::teachers;
use crate::utils::password;

const MIN_PASSWORD_LENGTH: usize = 8;

// <CreateErrors>
#[derive(Debug)]
pub enum CreateErrors {
  Multiple(Vec<CreateError>),
  UnexpectedError,
}
// </CreateErrors>

// <CreateError>
#[derive(Debug)]
pub enum CreateError {
  EmailIsBlank,
  EmailIsInvalid,
  PasswordIsBlank,
  PasswordIsTooShort,
}

make_serializable!(CreateError {
    EmailIsBlank => "Email can't be blank",
    EmailIsInvalid => "Email is invalid",
    PasswordIsBlank => "Password can't be blank",
    // TODO: Give better feedback on password security
    PasswordIsTooShort => "Password is too short (minimum is 8 characters)",
});
// </CreateError>

// <TeacherValues>
#[derive(Insertable)]
#[table_name = "teachers"]
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

  pub fn call(self) -> Result<(), CreateErrors> {
    self.validate()?.insert_teacher()?.finish()
  }

  fn validate(self) -> Result<Self, CreateErrors> {
    let mut errors = vec![];

    // Email regex taken from the infamous https://stackoverflow.com/questions/201323/how-to-validate-an-email-address-using-a-regular-expression
    // This unwrap is safe, because, well, I tested it?
    let email_regex = Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$")
      .expect("Error parsing regular expression");
    if self.email.trim().is_empty() {
      errors.push(CreateError::EmailIsBlank);
    } else if !email_regex.is_match(&self.email) {
      errors.push(CreateError::EmailIsInvalid);
    }

    if self.password.trim().is_empty() {
      errors.push(CreateError::PasswordIsBlank);
    } else if self.password.trim().len() < MIN_PASSWORD_LENGTH {
      // TODO: Implement better password validation (maybe using zxcvbn-rs)
      errors.push(CreateError::PasswordIsTooShort);
    }

    if errors.is_empty() {
      Ok(self)
    } else {
      Err(CreateErrors::Multiple(errors))
    }
  }

  fn insert_teacher(self) -> Result<Self, CreateErrors> {
    let values = TeacherValues::new(
      &self.email,
      password::digest(&self.password).map_err(|error| {
        // Report unexpected errors from argon2
        report_unexpected_err!(error);
        CreateErrors::UnexpectedError
      })?,
    );

    match diesel::insert_into(teachers::table)
      .values(&values)
      .get_result::<Teacher>(self.db)
    {
      Ok(_) => Ok(self),
      // If the email is already taken we still want to pretend that the sign up
      // was successful - this is a security measure against email enumeration
      // https://blog.rapid7.com/2017/06/15/about-user-enumeration
      Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => Ok(self),
      // Handle unexpected database-level errors:
      Err(error) => handle_unexpected_err!(error, CreateErrors::UnexpectedError),
    }
  }

  fn finish(self) -> Result<(), CreateErrors> {
    Ok(())
  }
}
// </Create>

pub fn create(email: String, password: String, db: &PgConnection) -> Result<(), CreateErrors> {
  Create::new(email, password, db).call()
}

#[cfg(test)]
mod tests {
  use super::*;

  fn test_db() -> PgConnection {
    dotenv::dotenv().ok();
    let db_url = std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL is not set");
    PgConnection::establish(&db_url).expect("Failed to establish a database connection")
  }

  macro_rules! setup {
    () => {
      let test_connection = test_db();
      test_connection
        .begin_test_transaction()
        .expect("Failed to begin a test transaction");

      let user_email = String::from("valid@email.com");
      let user_password = String::from("password");
    };
  }

  #[test]
  fn fails_when_email_is_blank() {
    setup!();
    let result = create("".into(), user_password, &test_connection);
    assert!(result.is_err());
  }

  #[test]
  fn fails_when_email_is_invalid() {
    setup!();
    let result = create("invalid".into(), user_password, &test_connection);
    assert!(result.is_err());
  }

  #[test]
  fn fails_when_password_is_too_short() {
    setup!();
    let result = create(
      String::from("valid@email.com"),
      String::from("pass"),
      &test_db(),
    );
    assert!(result.is_err());
  }

  #[test]
  fn fails_when_password_is_blank() {
    setup!();
    let result = create(
      String::from("valid@email.com"),
      String::from(""),
      &test_db(),
    );
    assert!(result.is_err());
  }

  #[test]
  fn works_when_user_already_exists() {
    setup!();
    create(
      "valid@email.com".into(),
      "password".into(),
      &test_connection,
    );
  }
}
