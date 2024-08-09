
use entity::income::Income as Income;
use entity::income_view::IncomeView;
use rocket_db_pools::{sqlx, Connection};
use time::{PrimitiveDateTime, OffsetDateTime};

use crate::user_routes::AuthenticatedUser;
use crate::db::Logs;

#[post("/save_income?<obs>&<category_id>&<value>&<date>")]
pub async fn save_income(mut db: Connection<Logs>, category_id: Option<i64>, obs: Option<&str>, value: f64, date: &str, user: AuthenticatedUser) -> String {
    let now = OffsetDateTime::now_utc(); //.to_offset(offset!(-3))

    sqlx::query!("INSERT INTO incomes
        (obs, category_id, value, `date`, user_id, created_date)
        VALUES(?, ?, ?, ?, ?, ?)",
        obs, category_id, value, date, user.user_id, PrimitiveDateTime::new(now.date(), now.time()))
        .execute(db.as_mut()).await.unwrap();

    "ok".to_string()
}

#[get("/search_income")]
pub async fn search_income(mut db: Connection<Logs>, user: AuthenticatedUser) -> String {

    let stream: Vec<IncomeView> = sqlx::query_as!(IncomeView,
            "SELECT * FROM incomes_view WHERE user_id = ? ORDER BY date desc, id desc",
            user.user_id
        )
        .fetch_all(db.as_mut())
        .await.unwrap();

    serde_json::to_string(&stream).unwrap()
}

#[post("/edit_income?<id>&<obs>&<value>&<category_id>&<date>")]
pub async fn edit_income(mut db: Connection<Logs>,id: i64, obs: Option<&str>, value: Option<f64>, category_id: Option<i64>, date: Option<&str>, user: AuthenticatedUser) -> String {

    if obs.is_some() && value.is_some() && date.is_some() && category_id.is_some() {
        sqlx::query!("UPDATE incomes SET obs = ?, value = ?,  `date` = ?, category_id = ? WHERE id = ? and user_id = ?",
            obs, value, date, category_id, id, user.user_id).execute(db.as_mut()).await.unwrap();
    } else if obs.is_some() {
        sqlx::query!("UPDATE incomes SET obs = ? WHERE id = ? and user_id = ?",
            obs, id, user.user_id).execute(db.as_mut()).await.unwrap();
    } else if value.is_some() {
        sqlx::query!("UPDATE incomes SET category_id = ? WHERE id = ? and user_id = ?",
            category_id, id, user.user_id).execute(db.as_mut()).await.unwrap();
    } else if category_id.is_some() {
        sqlx::query!("UPDATE incomes SET value = ? WHERE id = ? and user_id = ?",
            value, id, user.user_id).execute(db.as_mut()).await.unwrap();
    } else if date.is_some() {
        sqlx::query!("UPDATE incomes SET date = ? WHERE id = ? and user_id = ?",
            date, id, user.user_id).execute(db.as_mut()).await.unwrap();
    }

    "ok".to_string()
}

#[get("/get_income?<id>")]
pub async fn get_income(mut db: Connection<Logs>, id: i64, user: AuthenticatedUser) -> String {
    let stream = sqlx::query_as!(IncomeView,
        "SELECT * FROM incomes_view WHERE user_id = ? AND id = ?",
        user.user_id, id
    )
    .fetch_one(db.as_mut())
    .await.unwrap();

    serde_json::to_string(&stream).unwrap()
}

#[post("/delete_income?<id>")]
pub async fn delete_income(mut db: Connection<Logs>,id: i64, user: AuthenticatedUser) -> String {

    sqlx::query!("DELETE from incomes WHERE id = ? and user_id = ?",
        id, user.user_id).execute(db.as_mut()).await.unwrap();

    "ok".to_string()
}