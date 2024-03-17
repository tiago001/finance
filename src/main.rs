#[macro_use] extern crate rocket;

use rocket::fs::FileServer;
// use rocket::request::FlashMessage;
use rocket::response::Redirect;
use rocket_db_pools::Database;
use rocket_db_pools::{sqlx, Connection};
use rocket_dyn_templates::Template;
use entity::settings::Settings;
use serde_json::json;

use finance::{db, user_routes, expense_routes, income_routes};
use finance::{user_routes::AuthenticatedUser, user_routes::redirect_to_login};

use rocket::request::FromRequest;
use rocket::Request;
use rocket::request;
use rocket::request::Outcome;
use rocket::http::Status;

#[derive(Debug)]
struct FetchMode(String);

#[derive(Debug)]
enum FetchModeError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for FetchMode {
    type Error = FetchModeError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        println!("{:?}", req.headers());

        let keys: Vec<_> = req.headers().get("sec-fetch-mode").collect();
        match keys.len() {
            0 => {
                let keys: Vec<_> = req.headers().get("x-requested-with").collect();
                match keys.len() {
                    0 => Outcome::Success(FetchMode("navigate".to_string())),
                    1 => {
                        if keys[0] == "XMLHttpRequest"{
                            Outcome::Success(FetchMode("cors".to_string()))
                        } else {
                            Outcome::Success(FetchMode("navigate".to_string()))
                        }
                    }
                    _ => Outcome::Error((Status::BadRequest, FetchModeError::Invalid)),
                }
            },
            1 => Outcome::Success(FetchMode(keys[0].to_string())),
            _ => Outcome::Error((Status::BadRequest, FetchModeError::Invalid)),
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .attach(db::Logs::init())
    .mount("/", routes![
            expense_routes::save_expense,
            expense_routes::get_expense,
            expense_routes::search_expenses, 
            expense_routes::search_expenses_category,
            expense_routes::edit_expense,
            expense_routes::delete_expense,
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
            user_routes::register,
            user_routes::save_settings,
            user_routes::get_settings,
            index,
            settings,
            searchexpenses,
            addexpenses,
            dashboard,
            editexpense,
            editincome,
            income
        ]
    ).register("/",catchers![unauthorized])
    .mount("/", FileServer::from("static")) // Enable for development
    .attach(Template::fairing())
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
async fn settings(mode: FetchMode, mut db: Connection<db::Logs>, user: AuthenticatedUser) -> Template {
    let stream = match sqlx::query_as!(Settings,
            "SELECT * FROM settings WHERE user_id = ?",
            user.user_id
        ).fetch_one(db.as_mut()).await {
            Ok(result) => result,
            Err(..) => Settings{user_id: 0, budget: None}
        };
    if mode.0 == "navigate" {
        Template::render("pages/extended/settings", json!({"username": user.name,"settings": stream}))
    } else {
        Template::render("pages/settings", json!({"username": user.name,"settings": stream}))
    }
}

#[get("/searchexpenses")]
async fn searchexpenses(mode: FetchMode, user: AuthenticatedUser) -> Template {
    if mode.0 == "navigate" {
        Template::render("pages/extended/search_expenses", json!({"username": user.name}))
    } else {
        Template::render("pages/search_expenses", json!({"username": user.name}))
    }
}

#[get("/addexpenses")]
async fn addexpenses(mode: FetchMode, user: AuthenticatedUser) -> Template {
    if mode.0 == "navigate" {
        Template::render("pages/extended/add_expense", json!({"username": user.name}))
    } else {
        Template::render("pages/add_expense", json!({"username": user.name}))
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
async fn editexpense(user: AuthenticatedUser) -> Template {
    Template::render("pages/edit_expense",json!({"username": user.name}))
}

#[get("/editincome")]
async fn editincome(user: AuthenticatedUser) -> Template {
    Template::render("pages/edit_income",json!({"username": user.name}))
}

#[get("/income")]
async fn income(mode: FetchMode, user: AuthenticatedUser) -> Template {
    if mode.0 == "navigate" {
        Template::render("pages/extended/income", json!({"username": user.name}))
    } else {
        Template::render("pages/income", json!({"username": user.name}))
    }
}

#[catch(401)]
fn unauthorized() -> Redirect {
    redirect_to_login()
}
