use thiserror::Error;

#[derive(PartialEq, Error, Debug)]
pub enum DbError {
  #[error("Record not found")]
  RecordNotFound,
  #[error("Couldn't find {0} with {1} = {2}")]
  NotFound(&'static str, &'static str, String),
  #[error("Unique constraint violation: {0}")]
  UniqueConstraintViolation(String),
  #[error("Foreign key constraint violation: {0}")]
  ForeignKeyConstraintViolation(String),
  #[error(transparent)]
  UnexpectedError(diesel::result::Error)
}

impl From<diesel::result::Error> for DbError {
  fn from(error: diesel::result::Error) -> Self {
    use diesel::result::DatabaseErrorKind;

    match error {
      diesel::result::Error::DatabaseError(DatabaseErrorKind::UniqueViolation, details) => (
        DbError::UniqueConstraintViolation(format!("{:?}", details))
      ),
      diesel::result::Error::DatabaseError(DatabaseErrorKind::ForeignKeyViolation, details) => (
        DbError::ForeignKeyConstraintViolation(format!("{:?}", details))
      ),
      diesel::result::Error::NotFound => DbError::RecordNotFound,
      error => DbError::UnexpectedError(error),
    }
  }
}
