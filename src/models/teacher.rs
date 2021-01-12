#[derive(Queryable)]
pub struct Teacher {
    pub id: i32,
    pub email: String,
    pub password_digest: String,
    pub auth_token: Option<String>,
}
