#[macro_use] extern crate rocket;

use rocket::fs::{relative, FileServer};
use serde::Serialize;
use rocket_db_pools::{sqlx, Database, Connection};
use entity::expense::Expense as Expense;
use entity::users::Users as Users;
use rocket::response::Redirect;
use finance::{db, user_routes, user_routes::AuthenticatedUser, expense_routes};

// #[post("/save_expense?<name>&<value>&<category>&<date>")]
// async fn save_expense(mut db: Connection<db::Logs>, name: &str, value: f64, category: &str, date: &str, user: AuthenticatedUser) -> String {
    
//     sqlx::query("INSERT INTO expenses
//     (name, value, category,date, user_id)
//     VALUES(?, ?, ?, ?, ?)")
//     .bind(name)
//     .bind(value)
//     .bind(category)
//     .bind(date)
//     .bind(user.user_id)
//     .execute(&mut *db).await.unwrap();

//     "ok".to_string()
// }

// #[post("/save_expense?<name>&<value>&<category>&<date>", rank = 2)]
// async fn save_expense_redirect(name: &str, value: f64, category: &str, date: &str) -> Redirect {
//     redirect_to_login()
// }


// #[get("/search_last_expenses")]
// async fn search_last_expenses(mut db: Connection<db::Logs>, user: AuthenticatedUser) -> String {
//     let stream = sqlx::query_as!(Expense,
//         "SELECT * FROM expenses WHERE user_id = ? ORDER BY date DESC, id DESC",
//         user.user_id
//     )
//     .fetch_all(&mut *db)
//     .await.unwrap();

//     serde_json::to_string(&stream).unwrap()
// }

// #[get("/search_last_expenses", rank = 2)]
// pub async fn search_last_expenses_redirect() -> Redirect {
//     redirect_to_login()
// }

// #[get("/search_expenses?<name>&<value1>&<value2>")]
// async fn search_expenses(mut db: Connection<db::Logs>, name: &str, value1: Option<&str>, value2: Option<&str>, user: AuthenticatedUser) -> String {

//     let mut stream: Vec<Expense> = vec![];
//     if name == "category" {
//         if value1 == Some("Indefinido") {
//             stream = sqlx::query_as!(Expense,
//                 "SELECT * FROM expenses WHERE user_id = ? ORDER BY date DESC, id DESC",
//                 user.user_id
//             )
//             .fetch_all(&mut *db)
//             .await.unwrap();
//         } else {
//             stream = sqlx::query_as!(Expense,
//                 "SELECT * FROM expenses WHERE category = ? and user_id = ? ORDER BY date DESC, id DESC",
//                 value1,
//                 user.user_id
//             )
//             .fetch_all(&mut *db)
//             .await.unwrap();
//         }
//     } else if name == "date" {
//         stream = sqlx::query_as!(Expense,
//             "SELECT * FROM expenses where `date` between ? and ? and user_id = ? ORDER BY date DESC, id DESC",
//             value1,
//             value2,
//             user.user_id
//         )
//         .fetch_all(&mut *db)
//         .await.unwrap();
//     } else if name == "last15" {
//         stream = sqlx::query_as!(Expense, "SELECT * FROM expenses WHERE user_id = ? ORDER BY date DESC, id DESC LIMIT 15", user.user_id)
//         .fetch_all(&mut *db)
//         .await.unwrap();
//     }

//     serde_json::to_string(&stream).unwrap()
// }

// #[get("/search_expenses?<name>&<value1>&<value2>", rank = 2)]
// pub async fn search_expenses_redirect(name: &str, value1: Option<&str>, value2: Option<&str>) -> Redirect {
//     redirect_to_login()
// }

// #[get("/search_expenses_category")]
// async fn search_expenses_category(mut db: Connection<db::Logs>, user: AuthenticatedUser) -> String {
//     #[derive(Serialize, Debug)]
//     struct Record {category: String, sum: Option<f64>}
//     // #[derive(Serialize, Debug)]
//     // struct Compras {name: String,sum: Option<f64>}

//     let stream = sqlx::query_as!(Record, "SELECT category, SUM(value) as sum FROM expenses WHERE user_id = ? GROUP BY category ORDER BY 2 DESC", user.user_id)
//         .fetch_all(&mut *db)
//         .await.unwrap();

//     serde_json::to_string(&stream).unwrap()
// }

// #[get("/search_expenses_category", rank = 2)]
// pub async fn search_expenses_category_redirect() -> Redirect  {
//     redirect_to_login()
// }

// #[get("/get_user_info")]
// async fn get_user_info(mut db: Connection<db::Logs>, user: AuthenticatedUser) -> String {
//     let mut user = sqlx::query_as!(Users, "SELECT id, email, password FROM users WHERE id = ?", user.user_id)
//         .fetch_one(&mut *db)
//         .await.unwrap();

//     user.password = "".to_string();

//     serde_json::to_string(&user).unwrap()
// }

// #[get("/get_user_info", rank = 2)]
// async fn get_user_info_redirect() -> Redirect {
//     redirect_to_login()
// }

// pub fn redirect_to_login() -> Redirect {
//     Redirect::to("/login.html")
// }

#[launch]
fn rocket() -> _ {
    rocket::build()
    .attach(db::Logs::init())
    .mount("/", routes![
            expense_routes::save_expense,
            expense_routes::save_expense_redirect,
            expense_routes::search_last_expenses,
            expense_routes::search_last_expenses_redirect,
            expense_routes::search_expenses, 
            expense_routes::search_expenses_redirect, 
            expense_routes::search_expenses_category,
            expense_routes::search_expenses_category_redirect,
            user_routes::get_user_info,
            user_routes::get_user_info_redirect,
            user_routes::create_account,
            user_routes::verify_account,
            user_routes::logout
        ]
    ).mount("/", FileServer::from(relative!("static")))
}