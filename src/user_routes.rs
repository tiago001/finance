use std::ops::Add;

use entity::users::Users;
use rocket::{response::{Flash, Redirect}, http::{CookieJar, Cookie}, request::{FromRequest, Outcome}, Request};
use rocket::form::Form;
use rocket_db_pools::{sqlx, Connection};
use rocket_dyn_templates::Template;
use rocket::request::FlashMessage;
use serde_json::json;
use time::{PrimitiveDateTime, OffsetDateTime};
use ring::rand::SecureRandom;
use ring::rand;

use argon2::{ Config, Variant, Version};

use rocket::http::Status;

use time::Duration;

use crate::error::Error;

use crate::db::Logs;

pub fn redirect_to_login() -> Redirect {
    Redirect::to("/login")
}

pub struct AuthenticatedUser {
    pub user_id: i64,
    pub name: String
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = anyhow::Error;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let cookies = req.cookies();
        let user_id_cookie = match get_user_id_cookie(cookies) {
            Some(result) => result,
            None => return Outcome::Error((Status::Unauthorized, Error::UnauthenticatedError.into()))
        };

        let logged_in_user_id = match user_id_cookie.value().parse::<i64>() {
            Ok(result) => result,
            Err(err) => return Outcome::Error((Status::Unauthorized, err.into()))
        };
        
        let logged_in_username = match get_user_name_cookie(cookies) {
            Some(result) => result,
            None => return Outcome::Error((Status::Unauthorized, Error::UnauthenticatedError.into()))
        };

        return Outcome::Success(AuthenticatedUser { user_id: logged_in_user_id, name: logged_in_username.value().to_string() });
    }
}

pub struct UnauthenticatedUser {}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UnauthenticatedUser {
    type Error = anyhow::Error;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let cookies = req.cookies();
        match get_user_id_cookie(cookies) {
            Some(..) => return Outcome::Forward(Status::PermanentRedirect),
            None => return Outcome::Success(UnauthenticatedUser{})
        };
    }
}

fn get_user_id_cookie<'a>(cookies: &'a CookieJar) -> Option<Cookie<'a>> {
    cookies.get_private("user_id")
}

fn set_user_id_cookie(cookies: & CookieJar, user_id: i64) {
    let now = OffsetDateTime::now_utc().add(Duration::hours(48));
    cookies.add_private(Cookie::build(("user_id", user_id.to_string())).expires(now).secure(true).build());
    // cookies.add_private(Cookie::build(("user_id", user_id.to_string())).expires(now).build()); // For testing
}

fn remove_user_id_cookie(cookies: & CookieJar) {
    cookies.remove_private(Cookie::from("user_id"));
}

fn get_user_name_cookie<'a>(cookies: &'a CookieJar) -> Option<Cookie<'a>> {
    cookies.get_private("name")
}

fn set_user_name_cookie(cookies: & CookieJar, name: String) {
    let now = OffsetDateTime::now_utc().add(Duration::hours(48));
    cookies.add_private(Cookie::build(("name", name.to_string())).expires(now).secure(true).build());
    // cookies.add_private(Cookie::build(("name", name.to_string())).expires(now).build()); // For testing
}

fn remove_user_name_cookie(cookies: & CookieJar) {
    cookies.remove_private(Cookie::from("name"));
}

#[post("/logout")]
pub async fn logout(cookies: & CookieJar<'_>) -> Flash<Redirect> {
    remove_user_id_cookie(cookies);
    remove_user_name_cookie(cookies);
    Flash::success(Redirect::to("/login"), "Logged out succesfully!")
}

fn login_error() -> Flash<Redirect>  {
    Flash::error(Redirect::to("/login"), "Incorrect email or password")
}

#[derive(FromForm)]
pub struct InfoLogin {
    pub email: String,
    pub password: String,
    pub name: Option<String>
}

#[post("/createaccount", data="<user_form>")]
pub async fn create_account(mut db: Connection<Logs>, user_form: Form<InfoLogin>) -> Flash<Redirect> {
    let user = user_form.into_inner();

    if user.email.is_empty() || user.password.is_empty() || user.name.is_none() {
        return Flash::error(Redirect::to("/register"), "Please enter a valid email and password");
    }

    let stored_user: Option<Users> = match sqlx::query_as!(Users,
        "SELECT id, email, password, name FROM users WHERE email = ?",
        user.email
    )
    .fetch_one(db.as_mut())
    .await{
        Ok(model) => Some(model),
        Err(_) => None
    };

    if stored_user.is_some() {
        return Flash::error(Redirect::to("/register"), "User already registered");
    }

    let mut salt = [0u8; 16];
    let rng = rand::SystemRandom::new();
    rng.fill(&mut salt).unwrap();

    let hash_config = Config {
        ad: &[],
        hash_length: 32,
        lanes: 1,
        mem_cost: 65536,
        secret: &[],
        time_cost: 1,
        variant: Variant::Argon2id,
        version: Version::Version13,
    };
    let hash = match argon2::hash_encoded(user.password.as_bytes(), &salt, &hash_config) {
        Ok(result) => {
            result
        },
        Err(_) => {
            return Flash::error(Redirect::to("/register"), "Issue creating account");
        }
    };

    sqlx::query!("INSERT INTO users (email, password, name) VALUES(?, ?, ?)",
        user.email, hash, user.name).execute(db.as_mut()).await.unwrap();

    Flash::success(Redirect::to("/login"), "Account created succesfully!")
}

#[post("/verifyaccount", data="<user_form>")]
pub async fn verify_account(mut db: Connection<Logs>, cookies: & CookieJar<'_>, user_form: Form<InfoLogin>) -> Flash<Redirect> {
    let user = user_form.into_inner();

    let stored_user = match sqlx::query_as!(Users,
            "SELECT id, email, password, name FROM users WHERE email = ?;",
            user.email
        )
        .fetch_one(db.as_mut())
        .await{
            Ok(model) => model,
            Err(_) => return login_error()
        };
    
    let is_password_correct = match argon2::verify_encoded(&stored_user.password, user.password.as_bytes()) {
        Ok(result) => result,
        Err(_) => {
            return Flash::error(Redirect::to("/login"), "Incorrect email or password")
        }
    };

    if !is_password_correct {
        return login_error();
    }

    let now = OffsetDateTime::now_utc();

    sqlx::query!("INSERT INTO login_history (user_id, `date`) VALUES(?, ?);",
        stored_user.id, PrimitiveDateTime::new(now.date(), now.time()))
        .execute(db.as_mut()).await.unwrap();

    set_user_id_cookie(cookies, stored_user.id);
    set_user_name_cookie(cookies, stored_user.name);
    Flash::success(Redirect::to("/"), "Logged in succesfully!")
}

#[get("/get_user_info")]
pub async fn get_user_info(mut db: Connection<Logs>, user: AuthenticatedUser) -> String {
    let user = sqlx::query_as!(Users, "SELECT id, email, password, name FROM users WHERE id = ?",
        user.user_id)
        .fetch_one(db.as_mut())
        .await.unwrap();

    serde_json::to_string(&user).unwrap()
}

#[get("/login")]
pub async fn login(flash: Option<FlashMessage<'_>>, _user: UnauthenticatedUser) -> Template {
    Template::render("login",json!({"message": flash.map(FlashMessage::into_inner)}))
}

#[get("/login", rank = 2)]
pub async fn login_logged_user(_user: AuthenticatedUser) -> Redirect {
    Redirect::to("/")
}

#[get("/register")]
pub async fn register(flash: Option<FlashMessage<'_>>, _user: UnauthenticatedUser) -> Template {
    Template::render("register",json!({"message": flash.map(FlashMessage::into_inner)}))
}

#[get("/register", rank = 2)]
pub async fn register_logged_user(_user: AuthenticatedUser) -> Redirect {
    Redirect::to("/")
}