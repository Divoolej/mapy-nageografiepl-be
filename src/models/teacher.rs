use chrono::{DateTime, Utc};

use crate::schema::teachers;

#[derive(Identifiable, Queryable, Debug)]
pub struct Teacher {
  pub id: i32,
  pub uuid: String,
  pub email: String,
  pub password_digest: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}
