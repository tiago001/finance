#[macro_use] extern crate rocket;

use rocket::http::Status;
use rocket::fs::FileServer;
use rocket::response::Redirect;
use rocket::request::FromRequest;
use rocket::{tokio, Request};
use rocket::request::{self, Outcome};
use rocket_db_pools::{Database, Connection, sqlx};
use rocket_dyn_templates::Template;

use serde_json::json;

use finance::{db::Logs, user_routes, expense_routes, income_routes, settings_routes, investment_routes};
use finance::{user_routes::AuthenticatedUser, user_routes::redirect_to_login};
use finance::stock_update;

use entity::{settings::Settings, categories::Categories, investments::Investment};

use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

use dotenv::dotenv;

#[derive(Debug)]
struct FetchMode(String);

#[derive(Debug)]
enum FetchModeError {
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for FetchMode {
    type Error = FetchModeError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {

        let keys: Vec<_> = req.headers().get("load-mode").collect();
        match keys.len() {
            0 => Outcome::Success(FetchMode("navigate".to_string())),
            1 => Outcome::Success(FetchMode(keys[0].to_string())),
            _ => Outcome::Error((Status::BadRequest, FetchModeError::Invalid)),
        }
    }
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    tokio::spawn(async {
        match run_scheduled_task().await {
            Ok(..) => println!("task scheduled successfully"),
            Err(e) => println!("error initiating task {}", e)
        };
    });
    
    rocket::build()
    .attach(Logs::init())
    .mount("/", routes![
            expense_routes::save_expense,
            expense_routes::get_expense,
            expense_routes::search_expenses, 
            expense_routes::search_expenses_category,
            expense_routes::edit_expense,
            expense_routes::delete_expense,
            expense_routes::get_balance,
            expense_routes::predict_category,
            income_routes::save_income,
            income_routes::search_income,
            income_routes::delete_income,
            income_routes::edit_income,
            income_routes::get_income,
            user_routes::get_user_info,
            user_routes::create_account,
            user_routes::verify_account,
            user_routes::logout,
            user_routes::login,
            user_routes::login_logged_user,
            user_routes::register,
            user_routes::register_logged_user,
            settings_routes::save_settings,
            settings_routes::get_settings,
            settings_routes::save_category,
            settings_routes::delete_category,
            index,
            settings,
            searchexpenses,
            addexpenses,
            dashboard,
            editexpense,
            editincome,
            income,
            investment,
            investment_routes::add_investment,
            investment_routes::search_investment,
            investment_routes::save_investment
        ]
    ).register("/",catchers![unauthorized])
    .mount("/", FileServer::from("static")) // Enable for development
    .attach(Template::fairing())

    
}

async fn run_scheduled_task() -> Result<(), JobSchedulerError>{
    let sched = JobScheduler::new().await?;

    sched.add(
        Job::new_async("0 1 * * * *", |_uuid, _l| {
            Box::pin(async move {
                match stock_update::update().await {
                    Ok(..) => println!("task completed successfully"),
                    Err(e) => println!("error running task {}", e)
                };
            })
        })?
    ).await?;

    sched.start().await?;

    Ok(())
}


#[get("/")]
async fn index(mode: FetchMode, user: AuthenticatedUser) -> Template {
    if mode.0 == "navigate" {
        Template::render("pages/extended/home", json!({"username": user.name}))
    } else {
        Template::render("pages/home", json!({"username": user.name}))
    }
}

#[get("/settings")]
async fn settings(mode: FetchMode, mut db: Connection<Logs>, user: AuthenticatedUser) -> Template {
    let stream = match sqlx::query_as!(Settings,
            "SELECT * FROM settings WHERE user_id = ?",
            user.user_id
        ).fetch_one(db.as_mut()).await {
            Ok(result) => result,
            Err(..) => Settings{user_id: 0, budget: None}
        };

    let categories: Vec<Categories> = match sqlx::query_as! {Categories,
        "SELECT * FROM categories WHERE user_id = ?",
        user.user_id}
        .fetch_all(db.as_mut())
        .await{
            Ok(result) => result,
            Err(..) => Vec::new()
        };
    if mode.0 == "navigate" {
        Template::render("pages/extended/settings", json!({"username": user.name,"settings": stream, "categories": categories}))
    } else {
        Template::render("pages/settings", json!({"username": user.name,"settings": stream, "categories": categories}))
    }
}

#[get("/searchexpenses")]
async fn searchexpenses(mut db: Connection<Logs>, mode: FetchMode, user: AuthenticatedUser) -> Template {
    let categories = sqlx::query_as!(Categories,
            "SELECT * FROM categories WHERE user_id = ? and category_type = 'expenses'",
            user.user_id
        )
        .fetch_all(db.as_mut())
        .await.unwrap();

    if mode.0 == "navigate" {
        Template::render("pages/extended/search_expenses", json!({"username": user.name, "categories": categories}))
    } else {
        Template::render("pages/expense/search_expenses", json!({"username": user.name, "categories": categories}))
    }
}

#[get("/addexpenses")]
async fn addexpenses(mut db: Connection<Logs>, mode: FetchMode, user: AuthenticatedUser) -> Template {
    let categories = sqlx::query_as!(Categories,
            "SELECT * FROM categories WHERE user_id = ? and category_type = 'expenses'",
            user.user_id
        )
        .fetch_all(db.as_mut())
        .await.unwrap();

    if mode.0 == "navigate" {
        Template::render("pages/extended/add_expense", json!({"username": user.name, "categories": categories}))
    } else {
        Template::render("pages/expense/add_expense", json!({"username": user.name, "categories": categories}))
    }
}

#[get("/dashboard")]
async fn dashboard(mode: FetchMode, user: AuthenticatedUser) -> Template {
    if mode.0 == "navigate" {
        Template::render("pages/extended/dashboard", json!({"username": user.name}))
    } else {
        Template::render("pages/dashboard", json!({"username": user.name}))
    }
}

#[get("/editexpense")]
async fn editexpense(mut db: Connection<Logs>, user: AuthenticatedUser) -> Template {
    let categories = sqlx::query_as!(Categories,
            "SELECT * FROM categories WHERE user_id = ? and category_type = 'expenses'",
            user.user_id
        )
        .fetch_all(db.as_mut())
        .await.unwrap();

    Template::render("pages/expense/edit_expense",json!({"username": user.name, "categories": categories}))
}

#[get("/editincome")]
async fn editincome(mut db: Connection<Logs>, user: AuthenticatedUser) -> Template {
    let categories = sqlx::query_as!(Categories,
        "SELECT * FROM categories WHERE user_id = ? and category_type = 'incomes'",
        user.user_id
    )
    .fetch_all(db.as_mut())
    .await.unwrap();

    Template::render("pages/income/edit_income",json!({"username": user.name, "categories": categories}))
}

#[get("/income")]
async fn income(mut db: Connection<Logs>, mode: FetchMode, user: AuthenticatedUser) -> Template {
    let categories = sqlx::query_as!(Categories,
        "SELECT * FROM categories WHERE user_id = ? and category_type = 'incomes'",
        user.user_id
    )
    .fetch_all(db.as_mut())
    .await.unwrap();

    if mode.0 == "navigate" {
        Template::render("pages/extended/income", json!({"username": user.name, "categories": categories}))
    } else {
        Template::render("pages/income/income", json!({"username": user.name, "categories": categories}))
    }
}

#[get("/investment")]              
async fn investment(mut db: Connection<Logs>, mode: FetchMode, user: AuthenticatedUser) -> Template {
    let investments: Vec<Investment> = sqlx::query_as!(Investment,
                "SELECT id, stock, name, quantity, user_id
                FROM investments where user_id = ?",
                user.user_id
            )
            .fetch_all(db.as_mut())
            .await.unwrap();

    if mode.0 == "navigate" {
        Template::render("pages/extended/investment", json!({"username": user.name, "investments": investments}))
    } else {
        Template::render("pages/investment/investment", json!({"username": user.name, "investments": investments}))
    }
}

#[catch(401)]
fn unauthorized() -> Redirect {
    redirect_to_login()
}
