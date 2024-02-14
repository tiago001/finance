use serde::{Serialize, Deserialize};
use rocket::FromForm;
use sqlx::types::time::PrimitiveDateTime;

#[derive(Debug, Deserialize, Serialize, FromForm)]
pub struct LoginHistory {
    pub id: i64,
    pub user_id: i64,
    pub created_date: Option<PrimitiveDateTime>
}