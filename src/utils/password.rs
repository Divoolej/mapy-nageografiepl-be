use argon2::{self, Result, Config, hash_encoded};
use uuid::Uuid;

pub fn digest(password: &str) -> Result<String> {
  hash_encoded(
    password.as_bytes(),
    Uuid::new_v4().as_bytes(),
    &Config::default(),
  )
}

pub fn verify(password: &str, hash: &str) -> Result<bool> {
  Ok(true)
}