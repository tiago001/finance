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
async fn search_last_expenses(mut db: Connection<Logs>, _user: AuthenticatedUser) -> String {
    let stream = sqlx::query_as!(Expense,
        "SELECT * FROM expenses ORDER BY date DESC, id DESC"
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
async fn search_expenses(mut db: Connection<Logs>, name: &str, value1: Option<&str>, value2: Option<&str>, _user: AuthenticatedUser) -> String {

    println!("{}", _user.user_id);

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

#[get("/search_expenses?<name>&<value1>&<value2>", rank = 2)]
pub async fn search_expenses_redirect(name: &str, value1: Option<&str>, value2: Option<&str>) -> Redirect {
    redirect_to_login()
}

#[get("/search_expenses_category")]
async fn search_expenses_category(mut db: Connection<Logs>, _user: AuthenticatedUser) -> String {
    #[derive(Serialize, Debug)]
    struct Record {category: String, sum: Option<f64>}
    // #[derive(Serialize, Debug)]
    // struct Compras {name: String,sum: Option<f64>}

    let stream = sqlx::query_as!(Record, "SELECT category, SUM(value) as sum FROM expenses GROUP BY category ORDER BY 2 DESC")
        .fetch_all(&mut *db)
        .await.unwrap();

    serde_json::to_string(&stream).unwrap()
}

#[get("/search_expenses_category", rank = 2)]
pub async fn search_expenses_category_redirect() -> Redirect {
    redirect_to_login()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .attach(Logs::init())
    .mount("/", routes![
            save_expense,
            search_last_expenses,
            search_last_expenses_redirect,
            search_expenses, 
            search_expenses_redirect, 
            search_expenses_category,
            search_expenses_category_redirect,
            create_account,
            verify_account,
            logout
        ]
    ).mount("/", FileServer::from(relative!("static")))
}


// LOGIN //

use argon2::Config;
use rocket::{response::{Flash, Redirect}, http::{CookieJar, Cookie}, request::{FromRequest, Outcome}, Request};
use rocket::form::Form;

#[derive(Debug, Deserialize, Serialize, FromForm)]
pub struct Users {
    pub id: i32,
    pub email: String,
    pub password: String
}

impl Default for Users {
    fn default() -> Users {
        Users {
            id: 0,
            email: "".to_string(),
            password: "".to_string()
        }
    }
}

pub fn redirect_to_login() -> Redirect {
    Redirect::to("/login.html")
}

pub struct AuthenticatedUser {
    pub user_id: i32
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = anyhow::Error;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let cookies = req.cookies();
        let user_id_cookie = match get_user_id_cookie(cookies) {
            Some(result) => result,
            None => return Outcome::Forward(())
        };

        let logged_in_user_id = match user_id_cookie.value()
            .parse::<i32>() {
                Ok(result) => result,
                Err(_err) => return Outcome::Forward(())
            };

        return Outcome::Success(AuthenticatedUser { user_id: logged_in_user_id });
    }
}

fn get_user_id_cookie<'a>(cookies: &'a CookieJar) -> Option<Cookie<'a>> {
    cookies.get_private("user_id")
}

fn set_user_id_cookie(cookies: & CookieJar, user_id: i32) {
    cookies.add_private(Cookie::new("user_id", user_id.to_string()));
}

fn remove_user_id_cookie(cookies: & CookieJar) {
    cookies.remove_private(Cookie::named("user_id"));
}

#[post("/logout")]
pub async fn logout(cookies: & CookieJar<'_>) -> Flash<Redirect> {
    remove_user_id_cookie(cookies);
    Flash::success(Redirect::to("/login.html"), "Logged out succesfully!")
}

fn login_error() -> Flash<Redirect> {
    Flash::error(Redirect::to("/login"), "Incorrect email or password")
}

pub const USER_PASSWORD_SALT: &[u8] = b"some_random_salt";

#[post("/createaccount", data="<user_form>")]
async fn create_account(mut db: Connection<Logs>, user_form: Form<Users>) -> Flash<Redirect> {
    // let db = conn.into_inner();
    let user = user_form.into_inner();

    if user.email.is_empty() || user.password.is_empty() {
        return Flash::error(Redirect::to("/signup"), "Please enter a valid email and password");
    }

    let hash_config = Config::default();
    let hash = match argon2::hash_encoded(user.password.as_bytes(), USER_PASSWORD_SALT, &hash_config) {
        Ok(result) => result,
        Err(_) => {
            return Flash::error(Redirect::to("/signup"), "Issue creating account");
        }
    };

    sqlx::query("INSERT INTO users (email, password) VALUES(?, ?);")
        .bind(user.email)
        .bind(hash)
        .execute(&mut *db).await.unwrap();

    // let active_user = Users::ActiveModel {
    //     username: Set(user.username),
    //     password: Set(hash),
    //     ..Default::default()
    // };

    // match active_user.insert(db).await {
    //     Ok(result) => result,
    //     Err(_) => {
    //         return Flash::error(Redirect::to("/signup"), "Issue creating account");
    //     }
    // };

    Flash::success(Redirect::to("/login"), "Account created succesfully!")
}

#[post("/verifyaccount", data="<user_form>")]
async fn verify_account(mut db: Connection<Logs>, cookies: & CookieJar<'_>, user_form: Form<Users>) -> Flash<Redirect> {
    // let db = conn.into_inner();
    let user = user_form.into_inner();

    let stored_user = match sqlx::query_as!(Users,
            "SELECT id, email, password FROM users WHERE email = ?;
            ",
            user.email
        )
        .fetch_one(&mut *db)
        .await{
            Ok(model) => model,
            Err(_) => return login_error()
        };

    // let stored_user = match Users::find()
    //     .filter(users::Column::Username.contains(&user.username))
    //     .one(db)
    //     .await {
    //         Ok(model_or_null) => {
    //             match model_or_null {
    //                 Some(model) => model,
    //                 None => {
    //                     return login_error();
    //                 }
    //             }
    //         },
    //         Err(_) => {
    //             return login_error();
    //         }
    //     };
    
    let is_password_correct = match argon2::verify_encoded(&stored_user.password, user.password.as_bytes()) {
        Ok(result) => result,
        Err(_) => {
            return Flash::error(Redirect::to("/login"), "Encountered an issue processing your account")
        }
    };

    if !is_password_correct {
        return login_error();
    }

    set_user_id_cookie(cookies, stored_user.id);
    Flash::success(Redirect::to("/"), "Logged in succesfully!")
}