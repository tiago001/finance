use serde::{Serialize, Deserialize};
use sqlx::types::time::Date;
use sqlx::types::time::PrimitiveDateTime;

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Expense { 
    pub id: i64,
    pub name: String,
    pub value: f64,
    pub category: String,
    pub date: Date,
    pub user_id: Option<i64>,
    pub created_date: Option<PrimitiveDateTime>
}