pub fn init() -> String {
  let mut port = String::from("3000");
  match std::env::var("PORT") {
    Ok(value) => port = value,
    Err(std::env::VarError::NotPresent) => (),
    Err(std::env::VarError::NotUnicode(_)) => {
      println!("The value of PORT variable is invalid! Using the default port 3000.");
    },
  }

  port
}
