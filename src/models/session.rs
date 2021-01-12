use chrono::prelude::*;
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct Session {
    #[serde(skip_serializing)]
    pub id: i32,
    pub owner_type: String,
    pub owner_uuid: String,
    pub access_token: String,
    pub access_token_expires_at: DateTime<Utc>,
    pub refresh_token: String,
    pub refresh_token_expires_at: DateTime<Utc>,
    #[serde(skip_serializing)]
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing)]
    pub updated_at: DateTime<Utc>,
}