#[macro_use] extern crate rocket;

use rocket::fs::{relative, FileServer};
use rocket_db_pools::Database;
use finance::{db, user_routes, expense_routes};
use rocket_dyn_templates::Template;

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
            expense_routes::delete_expense,
            expense_routes::delete_expense_redirect,
            user_routes::get_user_info,
            user_routes::get_user_info_redirect,
            user_routes::create_account,
            user_routes::verify_account,
            user_routes::logout,
            user_routes::login,
            user_routes::register,
            index,
            searchexpenses,
            addexpenses,
            addexpenses_redirect
        ]
    ).mount("/", FileServer::from(relative!("static"))
    ).attach(Template::fairing())
    // ).mount("/", FileServer::from(relative!("static")))
}

use rocket::request::FlashMessage;
use serde_json::json;
use rocket::response::Redirect;

use crate::{user_routes::AuthenticatedUser, user_routes::redirect_to_login};

#[get("/")]
pub async fn index(flash: Option<FlashMessage<'_>>) -> Template {
    println!("teste tera aaa");
    // println!("{:?}", flash.map(FlashMessage::into_inner));
    return Template::render("index",json!({"message": flash.map(FlashMessage::into_inner)}))
    // return Template::render("login", json!({"message": "teste"}));
}

#[get("/searchexpenses")]
pub async fn searchexpenses(flash: Option<FlashMessage<'_>>) -> Template {
    // println!("{:?}", flash.map(FlashMessage::into_inner));
    return Template::render("pages/search_expenses",json!({"message": flash.map(FlashMessage::into_inner)}))
    // return Template::render("login", json!({"message": "teste"}));
}

#[get("/addexpenses")]
pub async fn addexpenses(flash: Option<FlashMessage<'_>>, user: AuthenticatedUser) -> Template {
    // println!("{:?}", flash.map(FlashMessage::into_inner));
    return Template::render("pages/add_expense",json!({"message": flash.map(FlashMessage::into_inner)}))
    // return Template::render("login", json!({"message": "teste"}));
}

#[get("/addexpenses", rank = 2)]
pub async fn addexpenses_redirect() -> Redirect {
    redirect_to_login()
}