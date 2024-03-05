
use rocket_db_pools::Database;

#[derive(Database)]
#[database("mysql_logs")]
pub struct Logs(sqlx::MySqlPool);