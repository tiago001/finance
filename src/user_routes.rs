use argon2::Config;
use entity::users::Users;
use rocket::{response::{Flash, Redirect}, http::{CookieJar, Cookie}, request::{FromRequest, Outcome}, Request};
use rocket::form::Form;
use rocket_db_pools::{sqlx, Connection};

use crate::db::Logs;

pub fn redirect_to_login() -> Redirect {
    Redirect::to("/login.html")
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
            None => return Outcome::Forward(())
        };

        let logged_in_user_id = match user_id_cookie.value()
            .parse::<i64>() {
                Ok(result) => result,
                Err(_err) => return Outcome::Forward(())
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
    Flash::success(Redirect::to("/login.html"), "Logged out succesfully!")
}

fn login_error() -> Flash<Redirect> {
    Flash::error(Redirect::to("/login"), "Incorrect email or password")
}

pub const USER_PASSWORD_SALT: &[u8] = b"some_random_salt";

#[derive(FromForm)]
pub struct InfoLogin {
    pub email: String,
    pub password: String
}

#[post("/createaccount", data="<user_form>")]
pub async fn create_account(mut db: Connection<Logs>, user_form: Form<InfoLogin>) -> Flash<Redirect> {
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

    Flash::success(Redirect::to("/login.html"), "Account created succesfully!")
}

#[post("/verifyaccount", data="<user_form>")]
pub async fn verify_account(mut db: Connection<Logs>, cookies: & CookieJar<'_>, user_form: Form<InfoLogin>) -> Flash<Redirect> {
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

#[get("/get_user_info", rank = 2)]
pub async fn get_user_info_redirect() -> Redirect {
    redirect_to_login()
}