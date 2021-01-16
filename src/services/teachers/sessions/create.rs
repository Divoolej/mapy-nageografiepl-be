use diesel::prelude::*;
use diesel::result::Error;
use serde::{Serialize, Serializer};

use crate::models::{Session, Teacher};
use crate::schema::sessions;
use crate::utils::{password, token};
use crate::{handle_unexpected_err, make_serializable};
use chrono::{DateTime, Duration, Utc};
use uuid::Uuid;

// <CreateErrors>
#[derive(Debug)]
pub enum CreateErrors {
    UnexpectedError,
    Multiple(Vec<CreateError>),
}
// </CreateErrors>

// <CreateError>
#[derive(Debug)]
pub enum CreateError {
    EmailIsBlank,
    EmailNotFound,
    PasswordIsBlank,
    PasswordDoesntMatch,
}

make_serializable!(CreateError {
  EmailIsBlank => "Email can't be blank",
  PasswordIsBlank => "Password can't be blank",
  EmailNotFound => "Invalid email/password combination",
  PasswordDoesntMatch => "Invalid email/password combination"
});
// </CreateError>

// <SessionValues>
#[derive(Insertable)]
#[table_name = "sessions"]
struct SessionValues<'a> {
    uuid: String,
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
            uuid: Uuid::new_v4().to_string(),
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

    pub fn call(self) -> Result<Session, CreateErrors> {
        self.validate()?
            .get_teacher()?
            .authenticate()?
            .insert_session()?
            .finish()
    }

    fn validate(self) -> Result<Self, CreateErrors> {
        let mut errors = vec![];

        if self.email.trim().is_empty() {
            errors.push(CreateError::EmailIsBlank);
        }
        if self.password.trim().is_empty() {
            errors.push(CreateError::PasswordIsBlank);
        }

        if errors.is_empty() {
            Ok(self)
        } else {
            Err(CreateErrors::Multiple(errors))
        }
    }

    fn get_teacher(mut self) -> Result<Self, CreateErrors> {
        use crate::schema::teachers::dsl::*;

        match teachers
            .filter(email.eq(&self.email))
            .first::<Teacher>(self.db)
        {
            Ok(teacher) => {
                self.teacher = Some(teacher);
                Ok(self)
            }
            Err(Error::NotFound) => Err(CreateErrors::Multiple(vec![CreateError::EmailNotFound])),
            // Handle unexpected database-level errors:
            Err(error) => handle_unexpected_err!(error, CreateErrors::UnexpectedError),
        }
    }

    fn authenticate(self) -> Result<Self, CreateErrors> {
        // It's safe to unwrap the teacher because we ensure it's presence in #get_teacher method
        match password::verify(
            &self.password,
            &self.teacher.as_ref().unwrap().password_digest,
        ) {
            Ok(true) => Ok(self),
            Ok(false) => Err(CreateErrors::Multiple(vec![
                CreateError::PasswordDoesntMatch,
            ])),
            // Handle unexpected errors from argon2:
            Err(error) => handle_unexpected_err!(error, CreateErrors::UnexpectedError),
        }
    }

    fn insert_session(mut self) -> Result<Self, CreateErrors> {
        // It's safe to unwrap the teacher because we ensure it's presence in #get_teacher method
        let values = SessionValues::new(&self.teacher.as_ref().unwrap().uuid);

        match diesel::insert_into(sessions::table)
            .values(&values)
            .get_result::<Session>(self.db)
        {
            Ok(session) => {
                self.session = Some(session);
                Ok(self)
            }
            // Handle unexpected database-level errors:
            Err(error) => handle_unexpected_err!(error, CreateErrors::UnexpectedError),
        }
    }

    fn finish(self) -> Result<Session, CreateErrors> {
        // The unwrap is safe because session presence is ensured in #insert_session
        Ok(self.session.unwrap())
    }
}
// </Create>

pub fn create(email: String, password: String, db: &PgConnection) -> Result<Session, CreateErrors> {
    Create::new(email, password, db).call()
}
