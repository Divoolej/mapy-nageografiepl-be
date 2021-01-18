use rand::Rng;

const TOKEN_LENGTH: usize = 20;

pub fn generate() -> String {
  let mut rng = rand::thread_rng();
  let bytes: [u8; TOKEN_LENGTH] = rng.gen();

  base64::encode_config(bytes, base64::URL_SAFE)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn generate_token_has_correct_length() {
    assert!(generate().len() >= TOKEN_LENGTH)
  }
}
