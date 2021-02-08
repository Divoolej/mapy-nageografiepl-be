use db::prelude::run_migrations;

pub fn init() -> Result<(), String> {
  run_migrations()?;

  Ok(())
}
