use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Investment {
    pub id: i64,
    pub stock: String,
    pub name: Option<String>,
    pub quantity: i64,
    pub user_id: i64,
}