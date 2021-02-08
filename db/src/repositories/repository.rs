use crate::utils::types::DbConnection;

pub trait Repository<'a> {
  fn new(db: &'a DbConnection) -> Self;
}

