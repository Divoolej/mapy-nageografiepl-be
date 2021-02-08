use diesel::r2d2::{Pool, ConnectionManager};
use diesel::PgConnection;

pub type DbConnection = PgConnection;
pub type DbPool = Pool<ConnectionManager<DbConnection>>;
