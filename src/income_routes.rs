
use entity::income::Income as Income;
use rocket_db_pools::{sqlx, Connection};
use time::{PrimitiveDateTime, OffsetDateTime};

use crate::user_routes::AuthenticatedUser;
use crate::db;

#[post("/save_income?<obs>&<value>&<date>")]
pub async fn save_income(mut db: Connection<db::Logs>, obs: Option<&str>, value: f64, date: &str, user: AuthenticatedUser) -> String {
    let now = OffsetDateTime::now_utc(); //.to_offset(offset!(-3))

    sqlx::query!("INSERT INTO incomes
        (obs, value, `date`, user_id, created_date)
        VALUES(?, ?, ?, ?, ?)",
        obs, value, date, user.user_id, PrimitiveDateTime::new(now.date(), now.time()))
        .execute(&mut *db).await.unwrap();

    "ok".to_string()
}

#[get("/search_income")]
pub async fn search_income(mut db: Connection<db::Logs>, user: AuthenticatedUser) -> String {

    let stream: Vec<Income> = sqlx::query_as!(Income,
            "SELECT * FROM incomes WHERE user_id = ? ORDER BY date desc",
            user.user_id
        )
        .fetch_all(&mut *db)
        .await.unwrap();

    serde_json::to_string(&stream).unwrap()
}