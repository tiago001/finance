#[macro_use] extern crate rocket;

use rocket::fs::{relative, FileServer};
use serde::{Serialize, Deserialize};
use rocket_db_pools::{sqlx, Database, Connection};

use sqlx::types::time::Date;

#[derive(Database)]
#[database("mysql_logs")]
struct Logs(sqlx::MySqlPool);

#[post("/save_expense?<name>&<value>&<category>&<date>")]
async fn save_expense(mut db: Connection<Logs>, name: &str, value: f64, category: &str, date: &str) -> String {
    
    sqlx::query("INSERT INTO expenses
    (name, value, category,date)
    VALUES(?, ?, ?, ?)")
    .bind(name)
    .bind(value)
    .bind(category)
    .bind(date)
    .execute(&mut *db).await.unwrap();

    "ok".to_string()
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
struct Expense { 
    id: i64,
    name: String,
    value: f64,
    category: String,
    date: Date
}

#[get("/search_last_expenses")]
async fn search_last_expenses(mut db: Connection<Logs>) -> String {
    let stream = sqlx::query_as!(Expense,
        "SELECT * FROM expenses ORDER BY date DESC, id DESC"
    )
    .fetch_all(&mut *db)
    .await.unwrap();

    serde_json::to_string(&stream).unwrap()
}

#[get("/search_expenses?<name>&<value1>&<value2>")]
async fn search_expenses(mut db: Connection<Logs>, name: &str, value1: Option<&str>,  value2: Option<&str>) -> String {
    let mut stream: Vec<Expense> = vec![];
    if name == "category" {
        if value1 == Some("Indefinido") {
            stream = sqlx::query_as!(Expense,
                "SELECT * FROM expenses ORDER BY date DESC, id DESC"
            )
            .fetch_all(&mut *db)
            .await.unwrap();
        } else {
            stream = sqlx::query_as!(Expense,
                "SELECT * FROM expenses WHERE category = ? ORDER BY date DESC, id DESC",
                value1
            )
            .fetch_all(&mut *db)
            .await.unwrap();
        }
    } else if name == "date" {
        stream = sqlx::query_as!(Expense,
            "SELECT * FROM expenses where `date` between ? and ? ORDER BY date DESC, id DESC",
            value1,
            value2
        )
        .fetch_all(&mut *db)
        .await.unwrap();
    } else if name == "last15" {
        stream = sqlx::query_as!(Expense, "SELECT * FROM expenses ORDER BY date DESC, id DESC LIMIT 15")
        .fetch_all(&mut *db)
        .await.unwrap();
    }

    serde_json::to_string(&stream).unwrap()
}


#[get("/search_expenses_category")]
async fn search_expenses_category(mut db: Connection<Logs>) -> String {
    #[derive(Serialize, Debug)]
    struct Record {category: String, sum: Option<f64>}
    // #[derive(Serialize, Debug)]
    // struct Compras {name: String,sum: Option<f64>}

    let stream = sqlx::query_as!(Record, "SELECT category, SUM(value) as sum FROM expenses GROUP BY category ORDER BY 2 DESC")
        .fetch_all(&mut *db)
        .await.unwrap();

    serde_json::to_string(&stream).unwrap()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .attach(Logs::init())
    .mount("/", routes![save_expense, search_last_expenses, search_expenses, search_expenses_category])
    .mount("/", FileServer::from(relative!("static")))
}