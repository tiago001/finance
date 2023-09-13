use argon2::Config;
use entity::{users::Users, settings::Settings};
use rocket::{response::{Flash, Redirect}, http::{CookieJar, Cookie}, request::{FromRequest, Outcome}, Request};
use rocket::form::Form;
use rocket_db_pools::{sqlx, Connection};
use rocket_dyn_templates::Template;
use rocket::request::FlashMessage;
use serde_json::json;

use rocket::http::Status;

use crate::error::Error;

use crate::db::{self, Logs};

pub fn redirect_to_login() -> Redirect {
    Redirect::to("/login")
}

pub struct AuthenticatedUser {
    pub user_id: i64
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = anyhow::Error;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let cookies = req.cookies();
        let user_id_cookie = match get_user_id_cookie(cookies) {
            Some(result) => result,
            None => return Outcome::Failure((Status::Unauthorized, Error::UnauthenticatedError.into()))
        };

        let logged_in_user_id = match user_id_cookie.value()
            .parse::<i64>() {
                Ok(result) => result,
                Err(err) => return Outcome::Failure((Status::Unauthorized, err.into()))
            };

        return Outcome::Success(AuthenticatedUser { user_id: logged_in_user_id });
    }
}

fn get_user_id_cookie<'a>(cookies: &'a CookieJar) -> Option<Cookie<'a>> {
    cookies.get_private("user_id")
}

fn set_user_id_cookie(cookies: & CookieJar, user_id: i64) {
    cookies.add_private(Cookie::new("user_id", user_id.to_string()));
}

fn remove_user_id_cookie(cookies: & CookieJar) {
    cookies.remove_private(Cookie::named("user_id"));
}

#[post("/logout")]
pub async fn logout(cookies: & CookieJar<'_>) -> Flash<Redirect> {
    remove_user_id_cookie(cookies);
    Flash::success(Redirect::to("/login"), "Logged out succesfully!")
}

fn login_error() -> Flash<Redirect>  {
    Flash::error(Redirect::to("/login"), "Incorrect email or password")
}

pub const USER_PASSWORD_SALT: &[u8] = b"some_random_salt";

#[derive(FromForm)]
pub struct InfoLogin {
    pub email: String,
    pub password: String,
    pub name: Option<String>
}

#[post("/createaccount", data="<user_form>")]
pub async fn create_account(mut db: Connection<Logs>, user_form: Form<InfoLogin>) -> Flash<Redirect> {
    let user = user_form.into_inner();

    if user.email.is_empty() || user.password.is_empty() {
        return Flash::error(Redirect::to("/register"), "Please enter a valid email and password");
    }

    let stored_user: Option<Users> = match sqlx::query_as!(Users,
        "SELECT id, email, password FROM users WHERE email = ?;
        ",
        user.email
    )
    .fetch_one(&mut *db)
    .await{
        Ok(model) => Some(model),
        Err(_) => None
    };

    if stored_user.is_some() {
        return Flash::error(Redirect::to("/register"), "User already registered");
    }

    let hash_config = Config::default();
    let hash = match argon2::hash_encoded(user.password.as_bytes(), USER_PASSWORD_SALT, &hash_config) {
        Ok(result) => result,
        Err(_) => {
            return Flash::error(Redirect::to("/register"), "Issue creating account");
        }
    };

    sqlx::query("INSERT INTO users (email, password, name) VALUES(?, ?, ?);")
        .bind(user.email)
        .bind(hash)
        .bind(user.name)
        .execute(&mut *db).await.unwrap();

    Flash::success(Redirect::to("/login"), "Account created succesfully!")
}

#[post("/verifyaccount", data="<user_form>")]
pub async fn verify_account(mut db: Connection<Logs>, cookies: & CookieJar<'_>, user_form: Form<InfoLogin>) -> Flash<Redirect> {
    let user = user_form.into_inner();

    println!(" teste {}{}", user.email, user.password);

    let stored_user = match sqlx::query_as!(Users,
            "SELECT id, email, password FROM users WHERE email = ?;",user.email
        )
        .fetch_one(&mut *db)
        .await{
            Ok(model) => model,
            Err(_) => return login_error()
        };
    
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

#[get("/get_user_info")]
pub async fn get_user_info(mut db: Connection<Logs>, user: AuthenticatedUser) -> String {
    let mut user = sqlx::query_as!(Users, "SELECT id, email, password FROM users WHERE id = ?", user.user_id)
        .fetch_one(&mut *db)
        .await.unwrap();

    user.password = "".to_string();

    serde_json::to_string(&user).unwrap()
}

#[get("/login")]
pub async fn login(flash: Option<FlashMessage<'_>>) -> Template {
    Template::render("login",json!({"message": flash.map(FlashMessage::into_inner)}))
}

#[get("/register")]
pub async fn register(flash: Option<FlashMessage<'_>>) -> Template {
    Template::render("register",json!({"message": flash.map(FlashMessage::into_inner)}))
}

#[post("/save_settings?<budget>")]
pub async fn save_settings(mut db: Connection<db::Logs>,budget: Option<f64>, user: AuthenticatedUser) -> String {

    let stream = sqlx::query_as!(Settings,
            "SELECT * FROM settings WHERE user_id = ?",
            user.user_id
        )
        .fetch_all(&mut *db)
        .await.unwrap();

    if stream.is_empty() {
        sqlx::query("INSERT INTO settings (user_id, budget) VALUES(?, ?);")
            .bind(user.user_id)
            .bind(budget.unwrap())
            .execute(&mut *db).await.unwrap();
            println!("insert");
    } else {
        sqlx::query("UPDATE settings SET budget = ? WHERE user_id = ?")
            .bind(budget.unwrap()).bind(user.user_id).execute(&mut *db).await.unwrap();
        println!("update");
    }

    "ok".to_string()
}

#[get("/get_settings")]
pub async fn get_settings(mut db: Connection<db::Logs>, user: AuthenticatedUser) -> String {

    let stream = sqlx::query_as!(Settings,
            "SELECT * FROM settings WHERE user_id = ?",
            user.user_id
        )
        .fetch_one(&mut *db)
        .await.unwrap();

    serde_json::to_string(&stream).unwrap()
}