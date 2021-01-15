use chrono::prelude::*;
use serde::Serialize;

use crate::schema::sessions;

#[derive(Identifiable, Queryable, Serialize, Debug)]
pub struct Session {
    #[serde(skip_serializing)] pub id: i32,
    pub uuid: String,
    #[serde(skip_serializing)] pub owner_type: String,
    pub owner_uuid: String,
    pub refresh_token: String,
    pub refresh_token_expires_at: DateTime<Utc>,
    pub access_token: String,
    pub access_token_expires_at: DateTime<Utc>,
    #[serde(skip_serializing)] pub created_at: DateTime<Utc>,
    #[serde(skip_serializing)] pub updated_at: DateTime<Utc>,
}
