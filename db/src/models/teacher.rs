use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::schema::teachers;

#[derive(PartialEq, Identifiable, Queryable, Debug)]
pub struct Teacher {
  pub id: i32,
  pub uuid: String,
  pub email: String,
  pub password_digest: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name = "teachers"]
pub struct NewTeacher {
  pub uuid: String,
  pub email: String,
  pub password_digest: String,
}

impl Default for NewTeacher {
  fn default() -> Self {
    Self {
      uuid: Uuid::new_v4().to_string(),
      email: String::new(),
      password_digest: String::new(),
    }
  }
}
