use argon2::{self, hash_encoded, verify_encoded, Config, Result};
use uuid::Uuid;

pub fn digest(password: &str) -> Result<String> {
  hash_encoded(password.as_bytes(), Uuid::new_v4().as_bytes(), &Config::default())
}

pub fn verify(password: &str, hash: &str) -> Result<bool> {
  verify_encoded(hash, password.as_bytes())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn digest_works() {
    let digested = digest("password");
    assert!(digested.is_ok());
    assert_eq!(digested.unwrap().is_empty(), false);
  }

  #[test]
  fn verify_works() {
    let digested = digest("password").unwrap();
    assert_eq!(verify("password", &digested), Ok(true));
    assert_eq!(verify("password1", &digested), Ok(false));
  }

  #[test]
  fn verify_with_empty_hash_fails() {
    assert!(verify("password", "").is_err());
  }
}
