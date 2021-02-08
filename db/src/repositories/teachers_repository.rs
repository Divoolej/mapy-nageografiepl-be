use diesel::prelude::*;
use diesel::result::Error;

use crate::utils::errors::DbError;
use crate::utils::types::DbConnection;
use crate::models::teacher::{Teacher, NewTeacher};
use crate::repositories::Repository;
use crate::schema;

pub struct TeachersRepository<'a> {
  db: &'a DbConnection,
}

impl<'a> Repository<'a> for TeachersRepository<'a> {
  fn new(db: &'a DbConnection) -> Self {
    Self { db }
  }
}

impl<'a> TeachersRepository<'a> {
  pub fn count(&self) -> Result<i64, DbError> {
    use diesel::dsl::count;
    use schema::teachers::dsl::*;

    teachers.select(count(id))
      .first(self.db)
      .map_err(|error| error.into())
  }

  pub fn find_by_email(&self, teacher_email: &str) -> Result<Teacher, DbError> {
    use schema::teachers::dsl::*;

    teachers.filter(email.eq(teacher_email))
      .first::<Teacher>(self.db)
      .map_err(|err| match err {
        Error::NotFound => DbError::RecordNotFound,
        error => error.into(),
      })
  }

  pub fn find_by_uuid(&self, teacher_uuid: &str) -> Result<Teacher, DbError> {
    use schema::teachers::dsl::*;

    teachers.filter(uuid.eq(teacher_uuid))
      .first::<Teacher>(self.db)
      .map_err(|err| match err {
        Error::NotFound => DbError::RecordNotFound,
        error => error.into(),
      })
  }

  pub fn create(&self, email: String, password_digest: String) -> Result<Teacher, DbError> {
    let new_teacher = NewTeacher { email, password_digest, ..Default::default() };

    diesel::insert_into(schema::teachers::table)
      .values(&new_teacher)
      .get_result::<Teacher>(self.db)
      .map_err(|error| error.into())
  }
}

#[cfg(test)]
mod tests {
  use serial_test::serial;
  use crate::utils::test::with_db;
  use super::*;

  #[test]
  #[serial]
  fn count_works() {
    with_db(|connection| {
      let count = TeachersRepository::new(&connection).count();
      assert!(count.is_ok());
      assert_eq!(count.unwrap(), 0);
    })
  }

  #[test]
  #[serial]
  fn create_works() {
    with_db(|connection| {
      let repository = TeachersRepository::new(&connection);
      assert!(repository.create("test".into(), "test".into()).is_ok());
      assert_eq!(repository.count().unwrap(), 1);
    })
  }

  #[test]
  #[serial]
  fn create_fails_when_email_is_taken() {
    with_db(|connection| {
      let repository = TeachersRepository::new(&connection);
      let email = "john.doe@example.com";
      repository.create(email.into(), "test1".into()).unwrap();

      match repository.create(email.into(), "test2".into()) {
        Err(DbError::UniqueConstraintViolation(_)) => (),
        _ => assert!(false),
      }
      assert_eq!(repository.count().unwrap(), 1);
    })
  }

  #[test]
  #[serial]
  fn find_by_email_works() {
    with_db(|connection| {
      let repository = TeachersRepository::new(&connection);
      let email = "john.doe@example.com";
      let teacher = repository.create(email.into(), "test1".into()).unwrap();

      let found_teacher = repository.find_by_email(email);
      assert!(found_teacher.is_ok());
      assert_eq!(found_teacher.unwrap().id, teacher.id);
    })
  }

  #[test]
  fn find_by_email_fails_when_teacher_doesnt_exist() {
    with_db(|connection| {
      let email = "john.doe@example.com";

      assert_eq!(
        TeachersRepository::new(&connection).find_by_email(email),
        Err(DbError::RecordNotFound),
      );
    })
  }

  #[test]
  #[serial]
  fn find_by_uuid_works() {
    with_db(|connection| {
      let repository = TeachersRepository::new(&connection);
      let teacher = repository.create("john.doe@example.com".into(), "test1".into()).unwrap();

      let found_teacher = repository.find_by_uuid(&teacher.uuid);
      assert!(found_teacher.is_ok());
      assert_eq!(found_teacher.unwrap().id, teacher.id);
    })
  }

  #[test]
  fn find_by_uuid_fails_when_teacher_doesnt_exist() {
    with_db(|connection| {
      let uuid = "some-uuid";

      assert_eq!(
        TeachersRepository::new(&connection).find_by_uuid(uuid),
        Err(DbError::RecordNotFound),
      );
    })
  }
}
