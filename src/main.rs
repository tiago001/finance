#[macro_use] extern crate rocket;

use rocket::fs::{relative, FileServer};
use rocket::request::FlashMessage;
// use rocket::request::FlashMessage;
use rocket::response::Redirect;
use rocket_db_pools::Database;
use rocket_db_pools::{sqlx, Connection};
use rocket_dyn_templates::Template;
use entity::settings::Settings;
use serde_json::json;

use finance::{db, user_routes, expense_routes};
use finance::{user_routes::AuthenticatedUser, user_routes::redirect_to_login};

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
            editexpense
        ]
    ).register("/",catchers![unauthorized])
    .mount("/", FileServer::from(relative!("static")))
    .attach(Template::fairing())
}


#[get("/")]
pub async fn index(user: AuthenticatedUser) -> Template {
    Template::render("pages/home",json!({"username": user.name}))
}

#[get("/settings")]
pub async fn settings(mut db: Connection<db::Logs>, user: AuthenticatedUser) -> Template {
    let stream = match sqlx::query_as!(Settings,
            "SELECT * FROM settings WHERE user_id = ?",
            user.user_id
        ).fetch_one(&mut *db).await {
            Ok(result) => result,
            Err(..) => Settings{user_id: 0, budget: None}
        };
    Template::render("pages/settings",json!({"username": user.name,"settings": stream}))
}

#[get("/searchexpenses")]
pub async fn searchexpenses(user: AuthenticatedUser) -> Template {
    Template::render("pages/search_expenses",json!({"username": user.name}))
}

#[get("/addexpenses")]
pub async fn addexpenses(user: AuthenticatedUser) -> Template {
    Template::render("pages/add_expense",json!({"username": user.name}))
}

#[get("/dashboard")]
pub async fn dashboard(user: AuthenticatedUser) -> Template {
    Template::render("pages/dashboard",json!({"username": user.name}))
}

#[get("/editexpense")]
pub async fn editexpense(user: AuthenticatedUser) -> Template {
    Template::render("pages/edit_expense",json!({"username": user.name}))
}

#[catch(401)]
fn unauthorized() -> Redirect {
    redirect_to_login()
}
