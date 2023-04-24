use serde::{Serialize, Deserialize};
use rocket::FromForm;

#[derive(Debug, Deserialize, Serialize, FromForm)]
pub struct Users {
    pub id: i64,
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