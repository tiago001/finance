use serde::{Serialize, Deserialize};
use rocket::FromForm;

#[derive(Debug, Deserialize, Serialize, FromForm)]
pub struct Users {
    pub id: i64,
    pub email: String,
    pub password: String,
    pub name: String,
    pub salt: Option<String>
}

impl Default for Users {
    fn default() -> Users {
        Users {
            id: 0,
            email: "".to_string(),
            password: "".to_string(),
            name: "".to_string(),
            salt: None
        }
    }
}