use env_logger;

pub fn init() -> Result<(), String> {
  env_logger::try_init().map_err(|err|
    format!("env_logger failed to initialize with the following error: {}", err)
  )?;

  Ok(())
}
