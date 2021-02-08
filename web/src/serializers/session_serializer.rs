use db::models::Session;

use crate::prelude::*;

#[derive(Serialize)]
pub struct SessionSerializer<'a> {
  uuid: &'a str,
  owner_uuid: &'a str,
  refresh_token: &'a str,
  refresh_token_expires_at: &'a DateTime<Utc>,
  access_token: &'a str,
  access_token_expires_at: &'a DateTime<Utc>,
}

impl<'a> From<&'a Session> for SessionSerializer<'a> {
  fn from(session: &'a Session) -> Self {
    SessionSerializer {
      uuid: &session.uuid,
      owner_uuid: &session.owner_uuid,
      refresh_token: &session.refresh_token,
      refresh_token_expires_at: &session.refresh_token_expires_at,
      access_token: &session.access_token,
      access_token_expires_at: &session.access_token_expires_at,
    }
  }
}
