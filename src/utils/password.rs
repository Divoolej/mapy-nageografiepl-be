use argon2::{self, hash_encoded, verify_encoded, Config, Result};
use uuid::Uuid;

pub fn digest(password: &str) -> Result<String> {
  hash_encoded(password.as_bytes(), Uuid::new_v4().as_bytes(), &Config::default())
}

pub fn verify(password: &str, hash: &str) -> Result<bool> {
  verify_encoded(hash, password.as_bytes())
}
