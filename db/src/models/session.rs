use chrono::{DateTime, Duration, Utc};
use uuid::Uuid;

use crate::schema::sessions;
use crate::utils::token;

#[derive(PartialEq, Identifiable, AsChangeset, Queryable, Debug)]
pub struct Session {
  pub id: i32,
  pub uuid: String,
  pub owner_type: String,
  pub owner_uuid: String,
  pub refresh_token: String,
  pub refresh_token_expires_at: DateTime<Utc>,
  pub access_token: String,
  pub access_token_expires_at: DateTime<Utc>,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name = "sessions"]
pub struct NewTeacherSession {
  pub uuid: String,
  pub owner_type: String,
  pub owner_uuid: String,
  pub refresh_token: String,
  pub refresh_token_expires_at: DateTime<Utc>,
  pub access_token: String,
  pub access_token_expires_at: DateTime<Utc>,
}

impl Default for NewTeacherSession {
  fn default() -> Self {
    Self {
      uuid: Uuid::new_v4().to_string(),
      owner_type: String::from("teacher"),
      owner_uuid: String::new(),
      refresh_token: token::generate(),
      refresh_token_expires_at: Utc::now() + Duration::weeks(4),
      access_token: token::generate(),
      access_token_expires_at: Utc::now() + Duration::days(1),
    }
  }
}

