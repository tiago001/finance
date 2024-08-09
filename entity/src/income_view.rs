use serde::{Serialize, Deserialize};
use sqlx::types::time::Date;
use sqlx::types::time::PrimitiveDateTime;

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct IncomeView { 
    pub id: i64,
    pub obs: Option<String>,
    pub value: f64,
    pub date: Date,
    pub user_id: i64,
    pub created_date: PrimitiveDateTime,
    pub category: Option<String>,
    pub category_id: Option<i64>
}