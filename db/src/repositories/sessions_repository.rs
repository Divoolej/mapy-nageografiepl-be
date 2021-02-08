use diesel::prelude::*;
use diesel::result::Error;

use crate::utils::errors::DbError;
use crate::utils::types::DbConnection;
use crate::models::Teacher;
use crate::models::session::{Session, NewTeacherSession};
use crate::repositories::Repository;
use crate::schema;

pub struct SessionsRepository<'a> {
  db: &'a DbConnection,
}

impl<'a> Repository<'a> for SessionsRepository<'a> {
  fn new(db: &'a DbConnection) -> Self {
    Self { db }
  }
}

impl<'a> SessionsRepository<'a> {
  pub fn count(&self) -> Result<i64, DbError> {
    use diesel::dsl::count;
    use schema::sessions::dsl::*;

    sessions.select(count(id))
      .first(self.db)
      .map_err(|error| error.into())
  }

  pub fn find_by_uuid(&self, session_uuid: &str) -> Result<Session, DbError> {
    use schema::sessions::dsl::*;

    sessions.filter(uuid.eq(session_uuid))
      .first::<Session>(self.db)
      .map_err(|err| match err {
        Error::NotFound => DbError::RecordNotFound,
        error => error.into(),
      })
  }

  pub fn create(&self, teacher: &Teacher) -> Result<Session, DbError> {
    let new_session = NewTeacherSession { owner_uuid: teacher.uuid.clone(), ..Default::default() };

    diesel::insert_into(schema::sessions::table)
      .values(&new_session)
      .get_result::<Session>(self.db)
      .map_err(|error| error.into())
  }

  pub fn save(&self, session: &Session) -> Result<Session, DbError> {
    diesel::update(session)
      .set(session)
      .get_result::<Session>(self.db)
      .map_err(|err| match err {
        Error::NotFound => (
          DbError::NotFound("session", "id", session.id.to_string())
        ),
        error => error.into(),
      })
  }

  pub fn destroy(&self, session: &Session) -> Result<(), DbError> {
    match diesel::delete(session).execute(self.db) {
      Ok(0) | Err(Error::NotFound) => (
        Err(DbError::NotFound("session", "id", session.id.to_string()))
      ),
      Ok(_) => Ok(()),
      Err(error) => Err(DbError::UnexpectedError(error)),
    }
  }
}

#[cfg(test)]
mod tests {
  use serial_test::serial;
  use crate::repositories::TeachersRepository;
  use crate::utils::test::with_db;
  use super::*;

  #[test]
  #[serial]
  fn count_works() {
    with_db(|connection| {
      let count = SessionsRepository::new(&connection).count();
      assert!(count.is_ok());
      assert_eq!(count.unwrap(), 0);
    })
  }

  #[test]
  #[serial]
  fn create_works() {
    with_db(|connection| {
      let teachers_repository = TeachersRepository::new(&connection);
      let sessions_repository = SessionsRepository::new(&connection);
      let teacher = teachers_repository.create("john.doe@example.com".into(), "test".into()).unwrap();

      assert!(sessions_repository.create(&teacher).is_ok());
      assert_eq!(sessions_repository.count().unwrap(), 1);
    })
  }

  #[test]
  #[serial]
  fn find_by_uuid_works() {
    with_db(|connection| {
      let teachers_repository = TeachersRepository::new(&connection);
      let sessions_repository = SessionsRepository::new(&connection);
      let teacher = teachers_repository
        .create("john.doe@example.com".into(), "test".into())
        .unwrap();
      let session = sessions_repository.create(&teacher).unwrap();

      let found_session = sessions_repository.find_by_uuid(&session.uuid);
      assert!(found_session.is_ok());
      assert_eq!(found_session.unwrap().id, session.id);
    })
  }

  #[test]
  fn find_by_uuid_fails_when_session_doesnt_exist() {
    with_db(|connection| {
      assert_eq!(
        SessionsRepository::new(&connection).find_by_uuid("some_uuid"),
        Err(DbError::RecordNotFound)
      );
    })
  }

  #[test]
  #[serial]
  fn save_works() {
    with_db(|connection| {
      let teachers_repository = TeachersRepository::new(&connection);
      let sessions_repository = SessionsRepository::new(&connection);
      let teacher = teachers_repository.create("john.doe@example.com".into(), "test".into()).unwrap();
      let mut session = sessions_repository.create(&teacher).unwrap();
      let new_uuid = "new-uuid".to_string();
      session.owner_uuid = new_uuid.clone();

      let result = sessions_repository.save(&session);
      assert!(result.is_ok());
      assert_eq!(result.unwrap().owner_uuid, new_uuid);
    })
  }

  #[test]
  #[serial]
  fn save_fails_when_session_doesnt_exist() {
    with_db(|connection| {
      let teachers_repository = TeachersRepository::new(&connection);
      let sessions_repository = SessionsRepository::new(&connection);
      let teacher = teachers_repository.create("john.doe@example.com".into(), "test".into()).unwrap();
      let mut session = sessions_repository.create(&teacher).unwrap();
      session.id = session.id + 2137;

      assert_eq!(
        sessions_repository.save(&session),
        Err(DbError::NotFound("session", "id", session.id.to_string())),
      );
    })
  }

  #[test]
  #[serial]
  fn destroy_works() {
    with_db(|connection| {
      let teachers_repository = TeachersRepository::new(&connection);
      let sessions_repository = SessionsRepository::new(&connection);
      let teacher = teachers_repository.create("john.doe@example.com".into(), "test".into()).unwrap();
      let session = sessions_repository.create(&teacher).unwrap();

      assert!(sessions_repository.destroy(&session).is_ok());
      assert_eq!(sessions_repository.count().unwrap(), 0);
    })
  }

  #[test]
  #[serial]
  fn destroy_fails_when_session_doesnt_exists() {
    with_db(|connection| {
      let teachers_repository = TeachersRepository::new(&connection);
      let sessions_repository = SessionsRepository::new(&connection);
      let teacher = teachers_repository.create("john.doe@example.com".into(), "test".into()).unwrap();
      let mut session = sessions_repository.create(&teacher).unwrap();
      session.id = session.id + 2137;

      assert_eq!(
        sessions_repository.destroy(&session),
        Err(DbError::NotFound("session", "id", session.id.to_string())),
      );
      assert_eq!(sessions_repository.count().unwrap(), 1);
    })
  }
}
