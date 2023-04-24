
use rocket_db_pools::{sqlx, Database};

#[derive(Database)]
#[database("mysql_logs")]
pub struct Logs(sqlx::MySqlPool);