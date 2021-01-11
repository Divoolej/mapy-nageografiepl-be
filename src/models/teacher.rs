use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Teacher {
    pub id: i32,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_digest: String,
    #[serde(skip_serializing)]
    pub auth_token: Option<String>,
}
