#[macro_use] extern crate rocket;

use rocket::fs::{relative, FileServer};
use rocket_db_pools::Database;
use finance::{db, user_routes, expense_routes};

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
            expense_routes::edit_expense,
            expense_routes::edit_expense_redirect,
            user_routes::get_user_info,
            user_routes::get_user_info_redirect,
            user_routes::create_account,
            user_routes::verify_account,
            user_routes::logout
        ]
    ).mount("/", FileServer::from(relative!("static")))
}