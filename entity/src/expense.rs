use serde::{Serialize, Deserialize};
use sqlx::types::time::Date;

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Expense { 
    pub id: i64,
    pub name: String,
    pub value: f64,
    pub category: String,
    pub date: Date,
    pub user_id: Option<i64>
}