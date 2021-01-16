use rand::Rng;

pub fn generate() -> String {
  let mut bytes = [0u8; 20];
  let mut rng = rand::thread_rng();

  for x in bytes.iter_mut() {
    *x = rng.gen();
  }

  base64::encode_config(bytes, base64::URL_SAFE)
}
