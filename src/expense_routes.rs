use entity::expense::Expense as Expense;
use entity::users::Users as Users;
use rocket::response::Redirect;
use rocket_db_pools::{sqlx, Connection};
use serde::Serialize;

use crate::{user_routes, user_routes::AuthenticatedUser, user_routes::redirect_to_login};
use crate::db::{self, Logs};

#[post("/save_expense?<name>&<value>&<category>&<date>")]
pub async fn save_expense(mut db: Connection<db::Logs>, name: &str, value: f64, category: &str, date: &str, user: AuthenticatedUser) -> String {
    
    sqlx::query("INSERT INTO expenses
    (name, value, category,date, user_id)
    VALUES(?, ?, ?, ?, ?)")
    .bind(name)
    .bind(value)
    .bind(category)
    .bind(date)
    .bind(user.user_id)
    .execute(&mut *db).await.unwrap();

    "ok".to_string()
}

#[post("/save_expense?<name>&<value>&<category>&<date>", rank = 2)]
pub async fn save_expense_redirect(name: &str, value: f64, category: &str, date: &str) -> Redirect {
    redirect_to_login()
}

#[post("/edit_expense?<id>&<name>&<value>&<category>&<date>")]
pub async fn edit_expense(mut db: Connection<db::Logs>,id: i64, name: Option<&str>, value: Option<f64>, category: Option<&str>, date: Option<&str>, user: AuthenticatedUser) -> String {
    
    if Some(name).is_some() {
        sqlx::query("UPDATE expenses SET name=? WHERE id = ? and user_id = ?")
            .bind(name).bind(id).bind(user.user_id).execute(&mut *db).await.unwrap();
    } else if Some(value).is_some() {
        sqlx::query("UPDATE expenses SET value=? WHERE id = ? and user_id = ?")
        .bind(value).bind(id).bind(user.user_id).execute(&mut *db).await.unwrap();
    } else if Some(category).is_some() {
        sqlx::query("UPDATE expenses SET category=? WHERE id = ? and user_id = ?")
        .bind(category).bind(id).bind(user.user_id).execute(&mut *db).await.unwrap();
    } else if Some(date).is_some() {
        sqlx::query("UPDATE expenses SET date=? WHERE id = ? and user_id = ?")
        .bind(date).bind(id).bind(user.user_id).execute(&mut *db).await.unwrap();
    }

    "ok".to_string()
}

#[post("/edit_expense?<id>&<name>&<value>&<category>&<date>", rank = 2)]
pub async fn edit_expense_redirect(id: i64,name: &str, value: f64, category: &str, date: &str) -> Redirect {
    redirect_to_login()
}


#[get("/search_last_expenses")]
pub async fn search_last_expenses(mut db: Connection<db::Logs>, user: AuthenticatedUser) -> String {
    let stream = sqlx::query_as!(Expense,
        "SELECT * FROM expenses WHERE user_id = ? ORDER BY date DESC, id DESC",
        user.user_id
    )
    .fetch_all(&mut *db)
    .await.unwrap();

    serde_json::to_string(&stream).unwrap()
}

#[get("/search_last_expenses", rank = 2)]
pub async fn search_last_expenses_redirect() -> Redirect {
    redirect_to_login()
}

#[get("/search_expenses?<name>&<value1>&<value2>")]
pub async fn search_expenses(mut db: Connection<db::Logs>, name: &str, value1: Option<&str>, value2: Option<&str>, user: AuthenticatedUser) -> String {

    let mut stream: Vec<Expense> = vec![];
    if name == "category" {
        if value1 == Some("Indefinido") {
            stream = sqlx::query_as!(Expense,
                "SELECT * FROM expenses WHERE user_id = ? ORDER BY date DESC, id DESC",
                user.user_id
            )
            .fetch_all(&mut *db)
            .await.unwrap();
        } else {
            stream = sqlx::query_as!(Expense,
                "SELECT * FROM expenses WHERE category = ? and user_id = ? ORDER BY date DESC, id DESC",
                value1,
                user.user_id
            )
            .fetch_all(&mut *db)
            .await.unwrap();
        }
    } else if name == "date" {
        stream = sqlx::query_as!(Expense,
            "SELECT * FROM expenses where `date` between ? and ? and user_id = ? ORDER BY date DESC, id DESC",
            value1,
            value2,
            user.user_id
        )
        .fetch_all(&mut *db)
        .await.unwrap();
    } else if name == "last15" {
        stream = sqlx::query_as!(Expense, "SELECT * FROM expenses WHERE user_id = ? ORDER BY date DESC, id DESC LIMIT 15", user.user_id)
        .fetch_all(&mut *db)
        .await.unwrap();
    }

    serde_json::to_string(&stream).unwrap()
}

#[get("/search_expenses?<name>&<value1>&<value2>", rank = 2)]
pub async fn search_expenses_redirect(name: &str, value1: Option<&str>, value2: Option<&str>) -> Redirect {
    redirect_to_login()
}

#[get("/search_expenses_category")]
pub async fn search_expenses_category(mut db: Connection<db::Logs>, user: AuthenticatedUser) -> String {
    #[derive(Serialize, Debug)]
    struct Record {category: String, sum: Option<f64>}
    // #[derive(Serialize, Debug)]
    // struct Compras {name: String,sum: Option<f64>}

    let stream = sqlx::query_as!(Record, "SELECT category, SUM(value) as sum FROM expenses WHERE user_id = ? GROUP BY category ORDER BY 2 DESC", user.user_id)
        .fetch_all(&mut *db)
        .await.unwrap();

    serde_json::to_string(&stream).unwrap()
}

#[get("/search_expenses_category", rank = 2)]
pub async fn search_expenses_category_redirect() -> Redirect  {
    redirect_to_login()
}