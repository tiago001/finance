
use chrono::NaiveDate;
use entity::categories::Categories;
use entity::expense_view::ExpenseView;
use rocket_db_pools::{sqlx, Connection};
use std::fs::File;
use std::{error::Error, io, process};

use entity::settings::Settings;
use time::{PrimitiveDateTime, OffsetDateTime};
use rocket::http::Status;
use rocket_dyn_templates::Template;
use serde_json::json;

use rocket::serde::json::Json;

use crate::user_routes::AuthenticatedUser;

use crate::db::Logs;

#[post("/save_category?<id>&<name>&<category_type>")]
pub async fn save_category(mut db: Connection<Logs>, id: Option<i64>, name: String, category_type: String, user: AuthenticatedUser) -> Status {
    let now = OffsetDateTime::now_utc();

    if id.is_some() {
        sqlx::query!("UPDATE categories
            SET category = ?
            WHERE id= ? and user_id = ? and category_type = ?;",
            name, id, user.user_id, category_type)
            .execute(db.as_mut()).await.unwrap();
    } else {
        let stream = sqlx::query_as!(Categories,
                "SELECT * FROM categories WHERE user_id = ? and category = ? and category_type = ?",
                user.user_id, name, category_type
            )
            .fetch_all(db.as_mut())
            .await.unwrap();
    
        if stream.is_empty() {
            sqlx::query!("INSERT INTO categories
                (user_id, category, category_type, created_date)
                VALUES(?, ?, ?, ?);",
                user.user_id, name, category_type, PrimitiveDateTime::new(now.date(), now.time()))
                .execute(db.as_mut()).await.unwrap();
        } else {
            return Status::Conflict
        }
    }

    Status::Ok
}

#[post("/delete_category?<id>")]
pub async fn delete_category(mut db: Connection<Logs>, id: Option<i64>, user: AuthenticatedUser) -> Status {
    sqlx::query!("UPDATE expenses
        SET category_id = null
        WHERE user_id = ? and category_id = ?",
        user.user_id, id)
        .execute(db.as_mut()).await.unwrap();

    sqlx::query!("DELETE FROM categories WHERE id = ? and user_id = ?",
        id, user.user_id)
        .execute(db.as_mut()).await.unwrap();

    // TODO remover categoria das despesas e receitas
    
    Status::Ok
}

#[post("/save_settings?<budget>")]
pub async fn save_settings(mut db: Connection<Logs>, budget: Option<f64>, user: AuthenticatedUser) -> String {

    let stream = sqlx::query_as!(Settings,
            "SELECT * FROM settings WHERE user_id = ?",
            user.user_id
        )
        .fetch_all(db.as_mut())
        .await.unwrap();

    if stream.is_empty() {
        sqlx::query!("INSERT INTO settings (user_id, budget) VALUES(?, ?)",
            user.user_id, budget.unwrap())
            .execute(db.as_mut()).await.unwrap();
    } else {
        sqlx::query!("UPDATE settings SET budget = ? WHERE user_id = ?",
            budget.unwrap(), user.user_id).execute(db.as_mut()).await.unwrap();
    }

    "ok".to_string()
}

#[get("/get_settings")]
pub async fn get_settings(mut db: Connection<Logs>, user: AuthenticatedUser) -> String {

    let settings: Settings = match sqlx::query_as!(Settings,
            "SELECT * FROM settings WHERE user_id = ?",
            user.user_id
        )
        .fetch_one(db.as_mut())
        .await {
            Ok(result) => result,
            Err(..) => Settings{ user_id: user.user_id, budget: Some(0.0)}
        };

    // let categories: Vec<Categories> = match sqlx::query_as! {Categories,
    //     "SELECT * FROM categories WHERE user_id = ?",
    //     user.user_id}
    //     .fetch_all(db.as_mut())
    //     .await{
    //         Ok(result) => result,
    //         Err(..) => Vec::new()
    //     };

    // let json = json!({
    //     "settings": settings,
    //     "categories": categories,
    // });


    serde_json::to_string(&settings).unwrap()
}

#[get("/get_budget_categories")]
pub async fn get_budget_categories(mut db: Connection<Logs>, user: AuthenticatedUser) -> Template {
    let categories: Vec<Categories> = (sqlx::query_as! {Categories,
        "SELECT * FROM categories WHERE user_id = ? and category_type = 'expenses'",
        user.user_id}
        .fetch_all(db.as_mut())
        .await).unwrap_or_default();

    Template::render("pages/settings/budget_categorties", json!({"categories": categories}))
}


#[post("/save_budget_categories", format = "json", data = "<categories>")]
pub async fn save_budget_categories(mut _db: Connection<Logs>, categories: Json<Vec<Categories>>) -> Status {
    println!("{:?}", categories);

    Status::Ok
}

#[post("/import_expenses")]
pub async fn import_expenses(mut db: Connection<Logs>, user: AuthenticatedUser) -> Status {

    #[derive(Debug, serde::Deserialize)]
    struct Record {
        #[serde(alias = "Id")]
        id: Option<i64>,
        #[serde(alias = "Descrição")]
        description: String,
        #[serde(alias = "Categoria")]
        category: String,
        #[serde(alias = "Valor")]
        value: f64,
        #[serde(alias = "Data")]
        date: String, // Option<PrimitiveDateTime>
    }

    let file = File::open(r"C:\Users\Tiago\Documents\expenses.csv").expect("File not found");
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: Record = result.unwrap();
        println!("{:?}", record);

        // let stream = sqlx::query_as!(ExpenseView,
        //     "SELECT id, name, value, `date`, user_id, created_date, category, category_id
        //         FROM expenses_view
        //         where name = ?
        //         and `date` = ?
        //         and value = ?
        //         and category = ?",
        //     record.description, NaiveDate::parse_from_str(&record.date, "%d/%m/%Y").unwrap().to_string(), record.value, record.category
        // )
        // .fetch_optional(db.as_mut())
        // .await.unwrap();

        // if stream.is_none() {
            let category = sqlx::query_as!(Categories,
                "SELECT * FROM categories WHERE user_id = ? and category_type = 'expenses' and category = ?",
                user.user_id, record.category
            )
            .fetch_optional(db.as_mut())
            .await.unwrap();

            let category_id = if category.is_some() { Some(category.unwrap().id) } else {None};

            let now = OffsetDateTime::now_utc();

            // println!("{:?}", stream);
            sqlx::query!("INSERT INTO expenses
                (name, value, category_id, date, user_id, created_date)
                VALUES(?, ?, ?, ?, ?, ?)",
                record.description, record.value, category_id, NaiveDate::parse_from_str(&record.date, "%d/%m/%Y").unwrap().to_string(), user.user_id, PrimitiveDateTime::new(now.date(), now.time()))
                .execute(db.as_mut()).await.unwrap();
        // }

    }

    Status::Ok
}